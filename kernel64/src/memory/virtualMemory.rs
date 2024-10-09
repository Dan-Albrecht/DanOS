use core::fmt::Write;
use kernel_shared::{
    assemblyStuff::halt::haltLoop,
    haltLoopWithMessage,
    magicConstants::{
        PAGES_PER_TABLE, SIZE_OF_PAGE,
    },
    memoryHelpers::{alignUp, haltOnMisaligned, zeroMemory2},
    pageTable::{
        enums::*, pageBook::PageBook, pageDirectoryPointerTable::PageDirectoryPointerTable,
        pageDirectoryTable::PageDirectoryTable,
        pageTable::PageTable,
    },
    physicalMemory::PhysicalMemoryManager,
};

use crate::loggerWriteLine;

use super::dumbHeap::BootstrapDumbHeap;

pub struct VirtualMemoryManager {
    physical: PhysicalMemoryManager,
    pageBook: PageBook,
    bdh: BootstrapDumbHeap,
}

struct VirtualMemoryIndex {
    pub PML4: usize,
    pub PDPT: usize,
    pub PD: usize,
    pub PT: usize,
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
        }
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

        VirtualMemoryIndex {
            PML4: (address >> 39) & 0x1FF,
            PDPT: (address >> 30) & 0x1FF,
            PD: (address >> 21) & 0x1FF,
            PT: (address >> 12) & 0x1FF,
        }
    }

    pub fn map(
        &mut self,
        physicalAddress: usize,
        virtualAddress: usize,
        length: usize,
        executable: Execute,
        present: Present,
        writable: Writable,
        cachable: Cachable,
        us: UserSupervisor,
        wt: WriteThrough,
    ) {
        // Just tryingt to prove we can flip the page table
        const BUGBUG_SUBTRACT_FROM_VIRTUAL: usize = 0x22_5000;

        // BUGBUG: Need to handle the case when a data structure already exists with conflicting enum bits
        haltOnMisaligned("Map - Physical", physicalAddress, SIZE_OF_PAGE);
        haltOnMisaligned("Map - Virtual", virtualAddress, SIZE_OF_PAGE);
        let adjustedLength = alignUp(length, SIZE_OF_PAGE);
        if adjustedLength != length {
            loggerWriteLine!("Wasted 0x{:X} in mapping", adjustedLength - length);
        }

        let length = adjustedLength;
        let vmi = Self::getVmi(virtualAddress);
        let numberOfPages = length / SIZE_OF_PAGE;

        if vmi.PT + numberOfPages > PAGES_PER_TABLE {
            // BUGUBG: Handle it
            haltLoopWithMessage!("Request crosses page directoris and we cannot handle that yet");
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
            let pml4 = self.pageBook.getEntry();
            if pml4.is_null() {
                haltLoopWithMessage!("No PML4");
            }

            let mut pdpt = (*pml4).getAddressForEntry(vmi.PML4);
            if pdpt.is_null() {
                let ptr = self
                    .bdh
                    .allocate(size_of::<PageDirectoryPointerTable>(), 0x1000);

                pdpt = ptr as *mut PageDirectoryPointerTable;
                zeroMemory2(pdpt);

                let phys = self.bdh.vToP(ptr) as *mut PageDirectoryPointerTable;
                (*pml4).setEntry(
                    vmi.PML4, phys, executable, present, writable, cachable, us, wt,
                );

                loggerWriteLine!("Allocated a new PDPT @ 0x{:X} / 0x{:X} (P/V)", phys as usize, ptr);
            } else {
                let old = pdpt;
                pdpt = self.bdh.pToV(pdpt as usize) as *mut PageDirectoryPointerTable;

                loggerWriteLine!("PDPT exists @ 0x{:X} / 0x{:X} (P/V)", old as usize, pdpt as usize);
            }

            let mut pdt = (*pdpt).getAddressForEntry(vmi.PDPT);
            if pdt.is_null() {
                let ptr = self.bdh.allocate(size_of::<PageDirectoryTable>(), 0x1000);

                pdt = ptr as *mut PageDirectoryTable;
                zeroMemory2(pdt);

                let phys = self.bdh.vToP(ptr) as *mut PageDirectoryTable;
                (*pdpt).setEntry(
                    vmi.PDPT, phys, executable, present, writable, cachable, us, wt,
                );

                loggerWriteLine!("Allocated a new PDT @ 0x{:X} / 0x{:X} (P/V)", phys as usize, ptr);
            } else {
                let old = pdt;
                pdt = self.bdh.pToV(pdt as usize) as *mut PageDirectoryTable;

                loggerWriteLine!("PDT exists @ 0x{:X} / 0x{:X} (P/V)", old as usize, pdt as usize);
            }

            let mut pt = (*pdt).getAddressForEntry(vmi.PD);
            if pt.is_null() {
                let ptr = self.bdh.allocate(size_of::<PageTable>(), 0x1000);

                pt = ptr as *mut PageTable;
                zeroMemory2(pt);

                let phys = self.bdh.vToP(ptr) as *mut PageTable;
                (*pdt).setEntry(
                    vmi.PD, phys, executable, present, writable, cachable, us, wt,
                );

                loggerWriteLine!("Allocated a new PT @ 0x{:X} / 0x{:X} (P/V)", phys as usize, ptr);
            } else {
                let old = pt;
                pt = self.bdh.pToV(pt as usize) as *mut PageTable;

                loggerWriteLine!("PT exists @ 0x{:X} / 0x{:X} (P/V)", old as usize, pt as usize);
            }

            for pageOffset in 0..numberOfPages {
                (*pt).setEntry(
                    vmi.PT + pageOffset,
                    (physicalAddress + (pageOffset * SIZE_OF_PAGE)) as u64,
                    executable,
                    present,
                    writable,
                    cachable,
                    us,
                    wt,
                );
            }
        }
    }

    pub fn identityMap(&mut self, 
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
        self.map(physicalAddress, physicalAddress, length, executable, present, writable, cachable, us, wt);
    }

    pub(crate) fn getPhysical(&self, address: usize) -> Option<usize> {
        let vmi = Self::getVmi(address);

        let pml4 = self.pageBook.getEntry();

        if pml4.is_null() {
            return None;
        }

        unsafe {
            let pdpt = (*pml4).getAddressForEntry(vmi.PML4);

            if pdpt.is_null() {
                return None;
            }

            let pdt = (*pdpt).getAddressForEntry(vmi.PDPT);

            if pdt.is_null() {
                return None;
            }

            let pt = (*pdt).getAddressForEntry(vmi.PD);

            if pt.is_null() {
                return None;
            }

            let pp = (*pt).getAddressForEntry(vmi.PT);

            return Some(pp as usize);
        }
    }
}
