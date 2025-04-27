use core::array::from_fn;
use kernel_shared::{
    assemblyStuff::halt::haltLoop,
    haltLoopWithMessage, loggerWrite,
    magicConstants::{PAGES_PER_TABLE, SIZE_OF_PAGE},
    memoryHelpers::{alignUp, haltOnMisaligned, zeroMemory2},
    memoryTypes::{PhysicalAddress, SomeSortOfIndex, VirtualAddress},
    pageTable::{
        enums::*, pageBook::PageBook, pageDirectoryPointerTable::PageDirectoryPointerTable,
        pageDirectoryTable::PageDirectoryTable, pageMapLevel4Table::PageMapLevel4Table,
        pageTable::PageTable, physicalPage::PhysicalPage,
    },
    physicalMemory::PhysicalMemoryManager,
};

use crate::loggerWriteLine;

use super::dumbHeap::BootstrapDumbHeap;

pub struct VirtualMemoryManager {
    physical: PhysicalMemoryManager,
    pageBook: PageBook,
    bdh: BootstrapDumbHeap,
    virtualAddresses: [usize; 100],
    nextVirtualAddressIndex: u8,
}

struct VirtualMemoryIndex {
    pub PML4: usize,
    pub PDPT: usize,
    pub PD: usize,
    pub PT: usize,
}

impl VirtualMemoryIndex {
    pub fn dump(&self) {
        loggerWriteLine!(
            "PML4: {}, PDPT: {}, PD: {}, PT: {}",
            self.PML4,
            self.PDPT,
            self.PD,
            self.PT
        );
    }
}

impl VirtualMemoryManager {
    pub fn new(
        physical: PhysicalMemoryManager,
        pageBook: PageBook,
        bdh: BootstrapDumbHeap,
    ) -> Self {
        VirtualMemoryManager {
            pageBook: pageBook,
            physical: physical,
            bdh: bdh,
            virtualAddresses: from_fn(|_| 0),
            nextVirtualAddressIndex: 0,
        }
    }

    pub fn dumpPhysical(&self) {
        self.physical.DumpBlobs();
    }

    fn is_canonical_address(virtual_address: usize) -> bool {
        let upper_bits = virtual_address >> 48;
        upper_bits == 0 || upper_bits == 0xFFFF
    }

    pub const fn canonicalize(address: usize) -> usize {
        let signBit = 1 << 47;
        if address & signBit != 0 {
            address | (!0 << 48)
        } else {
            address & ((1 << 48) - 1)
        }
    }

    fn getVmi(address: usize) -> VirtualMemoryIndex {
        if !Self::is_canonical_address(address) {
            haltLoopWithMessage!("0x{:X} is not canonical", address);
        }

        let result = VirtualMemoryIndex {
            PML4: (address >> 39) & 0x1FF,
            PDPT: (address >> 30) & 0x1FF,
            PD: (address >> 21) & 0x1FF,
            PT: (address >> 12) & 0x1FF,
        };

        result.dump();

        result
    }

    pub fn map(
        &mut self,
        mut physicalAddress: usize,
        mut virtualAddress: usize,
        length: usize,
        executable: Execute,
        present: Present,
        writable: Writable,
        cachable: Cachable,
        us: UserSupervisor,
        wt: WriteThrough,
    ) {
        let adjustedLength = alignUp(length, SIZE_OF_PAGE);
        if adjustedLength != length {
            loggerWriteLine!("Wasted 0x{:X} in mapping", adjustedLength - length);
        }

        let length = adjustedLength;
        let mut numberOfPages = length / SIZE_OF_PAGE;
        loggerWriteLine!(
            "Mapping 0x{:X} to 0x{:X} for 0x{:X}",
            physicalAddress,
            virtualAddress,
            length
        );

        while numberOfPages != 0 {
            let pagesMapped = self.mapInternal(
                physicalAddress,
                virtualAddress,
                numberOfPages,
                executable,
                present,
                writable,
                cachable,
                us,
                wt,
            );

            if numberOfPages == pagesMapped {
                break;
            }

            if pagesMapped > numberOfPages {
                haltLoopWithMessage!(
                    "Mapped {} pages, but requested {}",
                    pagesMapped,
                    numberOfPages
                );
            }

            numberOfPages -= pagesMapped;
            physicalAddress += pagesMapped * SIZE_OF_PAGE;
            virtualAddress += pagesMapped * SIZE_OF_PAGE;
        }

        loggerWriteLine!("Mapping complete");
    }

    // Requests to map the inputs
    // This function may end up mapping less, in which case you'll need to call it again adjusted for what it has already mapped
    // This function will return the number of pages mapped, so on reinvocation the address should be offset by the number of pages mapped
    fn mapInternal(
        &mut self,
        physicalAddress: usize,
        virtualAddress: usize,
        mut numberOfPages: usize,
        executable: Execute,
        present: Present,
        writable: Writable,
        cachable: Cachable,
        us: UserSupervisor,
        wt: WriteThrough,
    ) -> usize {
        // BUGBUG: Need to handle the case when a data structure already exists with conflicting enum bits
        haltOnMisaligned("Map - Physical", physicalAddress, SIZE_OF_PAGE);
        haltOnMisaligned("Map - Virtual", virtualAddress, SIZE_OF_PAGE);

        let vmi = Self::getVmi(virtualAddress);

        if vmi.PT + numberOfPages > PAGES_PER_TABLE {
            loggerWrite!("Mapping {} pages, but only ", numberOfPages);

            let nn = PAGES_PER_TABLE - vmi.PT;
            let remainingPages = numberOfPages - nn;
            numberOfPages = nn;

            loggerWriteLine!(
                "{} available, {} will need to be tried again",
                numberOfPages,
                remainingPages
            );
        }

        loggerWriteLine!(
            "Requested 0x{:X} / 0x{:X} (P/V) will live at {}, {}, {}, {}..{}",
            physicalAddress,
            virtualAddress,
            vmi.PML4,
            vmi.PDPT,
            vmi.PD,
            vmi.PT,
            vmi.PT + numberOfPages
        );

        unsafe {
            let virtualPml4 = self.pageBook.getVirtual();
            if virtualPml4.is_null() {
                // We wouldn't have made it to 64-bit if this was present in the CPU, so this means there's something
                // wrong with the created page book
                haltLoopWithMessage!("No PML4 in PageBook!");
            }

            let mut physicalPdpt = (*virtualPml4.ptr()).getAddressForEntry(vmi.PML4);
            let virtualPdpt: VirtualAddress<PageDirectoryPointerTable>;

            if physicalPdpt.is_null() {
                virtualPdpt = self
                    .bdh
                    .allocate(size_of::<PageDirectoryPointerTable>(), 0x1000);

                zeroMemory2(virtualPdpt.ptr());

                physicalPdpt = self.bdh.vToP(&virtualPdpt);

                loggerWriteLine!(
                    "Allocated a new PDPT @ 0x{:X} / 0x{:X} (P/V)",
                    physicalPdpt.address,
                    virtualPdpt.address
                );

                (*virtualPml4.ptr()).setEntry(
                    vmi.PML4,
                    &physicalPdpt,
                    executable,
                    present,
                    writable,
                    cachable,
                    us,
                    wt,
                    SomeSortOfIndex { value: u8::MAX },
                );
            } else {
                virtualPdpt = self.bdh.pToV(&physicalPdpt);

                loggerWriteLine!(
                    "PDPT exists @ 0x{:X} / 0x{:X} (P/V)",
                    physicalPdpt.address,
                    virtualPdpt.address
                );
            }

            if physicalPdpt.address == virtualPdpt.address {
                loggerWriteLine!("PDPT is identity mapped");
            } else {
                loggerWriteLine!("PDPT is not identity mapped");
            }

            let mut physicalPdt = (*virtualPdpt.ptr()).getAddressForEntry(vmi.PDPT);
            let virtualPdt: VirtualAddress<PageDirectoryTable>;

            if physicalPdt.is_null() {
                virtualPdt = self
                    .bdh
                    .allocate::<PageDirectoryTable>(size_of::<PageDirectoryTable>(), 0x1000);
                zeroMemory2(virtualPdt.ptr());

                physicalPdt = self.bdh.vToP(&virtualPdt);

                (*virtualPdpt.ptr()).setEntry(
                    vmi.PDPT,
                    &physicalPdt,
                    executable,
                    present,
                    writable,
                    cachable,
                    us,
                    wt,
                );

                loggerWriteLine!(
                    "Allocated a new PDT @ 0x{:X} / 0x{:X} (P/V)",
                    physicalPdt.address,
                    virtualPdt.address
                );
            } else {
                virtualPdt = self.bdh.pToV(&physicalPdt);

                loggerWriteLine!(
                    "PDT exists @ 0x{:X} / 0x{:X} (P/V)",
                    physicalPdt.address,
                    virtualPdt.address
                );
            }

            let mut physicalPageTable = (*virtualPdt.ptr()).getAddressForEntry(vmi.PD);
            let virtualPageTable: VirtualAddress<PageTable>;

            if physicalPageTable.is_null() {
                virtualPageTable = self
                    .bdh
                    .allocate::<PageTable>(size_of::<PageTable>(), 0x1000);
                zeroMemory2(virtualPageTable.ptr());

                physicalPageTable = self.bdh.vToP(&virtualPageTable);

                (*virtualPdt.ptr()).setEntry(
                    vmi.PD,
                    &physicalPageTable,
                    executable,
                    present,
                    writable,
                    cachable,
                    us,
                    wt,
                );

                loggerWriteLine!(
                    "Allocated a new PT @ 0x{:X} / 0x{:X} (P/V)",
                    physicalPageTable.address,
                    virtualPageTable.address
                );
            } else {
                virtualPageTable = self.bdh.pToV(&physicalPageTable);

                loggerWriteLine!(
                    "PT exists @ 0x{:X} / 0x{:X} (P/V)",
                    physicalPageTable.address,
                    virtualPageTable.address
                );
            }

            for pageOffset in 0..numberOfPages {
                let pageAddress = physicalAddress + (pageOffset * SIZE_OF_PAGE);
                let pageAddress = PhysicalAddress::<PhysicalPage>::new(pageAddress);
                (*virtualPageTable.ptr()).setEntry(
                    vmi.PT + pageOffset,
                    &pageAddress,
                    executable,
                    present,
                    writable,
                    cachable,
                    us,
                    wt,
                );
            }

            numberOfPages
        }
    }

    pub fn identityMap(
        &mut self,
        physicalAddress: usize,
        length: usize,
        executable: Execute,
        present: Present,
        writable: Writable,
        cachable: Cachable,
        us: UserSupervisor,
        wt: WriteThrough,
    ) {
        haltOnMisaligned("Identity map", physicalAddress, SIZE_OF_PAGE);
        self.map(
            physicalAddress,
            physicalAddress,
            length,
            executable,
            present,
            writable,
            cachable,
            us,
            wt,
        );
    }

    pub(crate) fn mapPhysicalAnywhere(
        &self,
        physicalAddress: usize,
        length: usize,
        execute: Execute,
        present: Present,
        writable: Writable,
        cachable: Cachable,
        supervisor: UserSupervisor,
        writeTrough: WriteThrough,
    ) -> usize {
        haltOnMisaligned("mapPhysicalAnywhere", physicalAddress, SIZE_OF_PAGE);
        let allocationSize = alignUp(length, SIZE_OF_PAGE);

        if length != allocationSize {
            loggerWriteLine!("Request was {} short", allocationSize - length);
        }

        //let vmi = Self::getVmi(virtualAddress);
        let numberOfPages = length / SIZE_OF_PAGE;
        let virtualAddress = self.getFreeVirtualAddress(numberOfPages);

        0
    }

    fn getVirtualAddress<T>(&self, xxx: SomeSortOfIndex) -> VirtualAddress<T> {
        let index = xxx.value;
        if index >= self.nextVirtualAddressIndex {
            loggerWriteLine!("VMM dump:");
            for x in 0..10 {
                loggerWriteLine!("{} = 0x{:X}", x, self.virtualAddresses[x]);
            }
            self.bdh.debugDump();
            haltLoopWithMessage!("Index {} hasn't even been used yet", index);
        }

        VirtualAddress::new(self.virtualAddresses[index as usize])
    }

    fn setXxx(&mut self, address: usize) -> u8 {
        let result = self.nextVirtualAddressIndex;
        self.virtualAddresses[self.nextVirtualAddressIndex as usize] = address;
        self.nextVirtualAddressIndex += 1;

        result
    }

    pub fn getFreeVirtualAddress(&self, numberOfPages: usize) -> usize {
        loggerWriteLine!("Doing get");
        unsafe {
            let pml4 = self.pageBook.getPhysical();

            if pml4.is_null() {
                haltLoopWithMessage!("PML4 is unmapped");
            }

            // BUGBUG: This is wrong, might not always be 0 for xxx value
            let xxx = (*pml4.unsafePtr()).getSomeSortOfIndex(0);
            let pml4 = self.getVirtualAddress::<PageMapLevel4Table>(xxx).ptr();
            let pml4Entries = (*pml4).getNumberOfEntries();
            for pml4Index in 0..pml4Entries {
                let pdpt = (*pml4).getAddressForEntry(pml4Index);
                if !pdpt.is_null() {
                    loggerWriteLine!("Looking {}", pml4Index);
                    // BUGBUG: Should be virtual
                    let pdptEntries = (*pdpt.unsafePtr()).getNumberOfEntries();
                    for pdptIndex in 0..pdptEntries {
                        // BUGBUG: Should be virtual
                        let pdt = (*pdpt.unsafePtr()).getAddressForEntry(pdptIndex);
                        if !pdt.is_null() {
                            loggerWriteLine!("Looking {},{}", pml4Index, pdptIndex);
                            let pdtEntries = (*pdt.unsafePtr()).getNumberOfEntries();
                            for ptIndex in 0..pdtEntries {
                                let pt = (*pdt.unsafePtr()).getAddressForEntry(ptIndex);
                                if !pt.is_null() {
                                    loggerWriteLine!(
                                        "Looking {},{},{}",
                                        pml4Index,
                                        pdptIndex,
                                        ptIndex
                                    );
                                    let ptEntries = (*pt.unsafePtr()).getNumberOfEntries();
                                    for ppIndex in 0..ptEntries {
                                        let pp = (*pt.unsafePtr()).getAddressForEntry(ppIndex);
                                        if pp.is_null() {
                                            loggerWriteLine!(
                                                "Free {},{},{},{}",
                                                pml4Index,
                                                pdptIndex,
                                                ptIndex,
                                                ppIndex
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        0
    }
}
