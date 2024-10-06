use core::fmt::Write;
use kernel_shared::{
    assemblyStuff::halt::haltLoop,
    haltLoopWithMessage,
    magicConstants::{
        PAGES_PER_TABLE, SIZE_OF_PAGE, SIZE_OF_PAGE_DIRECTORY, SIZE_OF_PAGE_DIRECTORY_POINTER,
        SIZE_OF_PAGE_TABLE,
    },
    memoryHelpers::{alignDown, alignUp, haltOnMisaligned, zeroMemory2},
    pageTable::{
        enums::*, pageBook::PageBook, pageDirectoryPointerTable::PageDirectoryPointerTable,
        pageDirectoryTable::PageDirectoryTable, pageMapLevel4Table::PageMapLevel4Table,
        pageTable::PageTable,
    },
    physicalMemory::{PhysicalMemoryManager, WhatDo},
    vgaWriteLine,
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

            let mut pdpt = (*pml4).getAddressForEntry(vmi.PML4);
            if pdpt as usize == 0 {
                let ptr = self
                    .bdh
                    .allocate(size_of::<PageDirectoryPointerTable>(), 0x1000);
                loggerWriteLine!("Allocated a new PDPT @ 0x{:X}", ptr);

                pdpt = ptr as *mut PageDirectoryPointerTable;
                zeroMemory2(pdpt);

                (*pml4).setEntry(
                    vmi.PML4, pdpt, executable, present, writable, cachable, us, wt,
                );
            }

            let mut pdt = (*pdpt).getAddressForEntry(vmi.PDPT);
            if pdt as usize == 0 {
                let ptr = self.bdh.allocate(size_of::<PageDirectoryTable>(), 0x1000);
                loggerWriteLine!("Allocated a new PDT @ 0x{:X}", ptr);

                pdt = ptr as *mut PageDirectoryTable;
                zeroMemory2(pdt);

                (*pdpt).setEntry(
                    vmi.PDPT, pdt, executable, present, writable, cachable, us, wt,
                );
            }

            let mut pt = (*pdt).getAddressForEntry(vmi.PD);
            if pt as usize == 0 {
                let ptr = self.bdh.allocate(size_of::<PageTable>(), 0x1000);
                loggerWriteLine!("Allocated a new PT @ 0x{:X}", ptr);

                pt = ptr as *mut PageTable;
                zeroMemory2(pt);

                (*pdt).setEntry(vmi.PD, pt, executable, present, writable, cachable, us, wt);
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

    pub fn identityMap(&mut self, startAddress: usize, numberOfPages: usize, whatDo: WhatDo) {
        haltOnMisaligned("Identity map", startAddress, SIZE_OF_PAGE);

        self.physical.Reserve(startAddress, SIZE_OF_PAGE, whatDo);

        let pageDirectoryPointerIndex = startAddress / SIZE_OF_PAGE_DIRECTORY_POINTER;
        let pageDirectoryIndex =
            (startAddress % SIZE_OF_PAGE_DIRECTORY_POINTER) / SIZE_OF_PAGE_DIRECTORY;
        let pageTableIndex = (startAddress % SIZE_OF_PAGE_DIRECTORY) / SIZE_OF_PAGE_TABLE;
        let pageIndex = (startAddress % SIZE_OF_PAGE_TABLE) / SIZE_OF_PAGE;

        if pageIndex + numberOfPages > PAGES_PER_TABLE {
            // BUGUBG: Handle it
            haltLoopWithMessage!("Request crosses page directoris and we cannot handle that yet");
        }

        loggerWriteLine!(
            "Requested 0x{:X} will live at {}, {}, {}, {}",
            startAddress,
            pageDirectoryPointerIndex,
            pageDirectoryIndex,
            pageTableIndex,
            pageIndex,
        );

        Self::getVmi(startAddress);

        // BUGUBG: Don't be lazy
        if pageDirectoryPointerIndex != 0 {
            loggerWriteLine!(
                "Don't know how to do PDPT 0x{:X}",
                pageDirectoryPointerIndex
            );
            haltLoop();
        }

        unsafe {
            let pml4 = self.pageBook.getEntry();
            let pdpt = (*pml4).getAddressForEntry(pageDirectoryPointerIndex);

            let mut pdt = (*pdpt).getAddressForEntry(pageDirectoryIndex);
            if pdt as usize == 0 {
                loggerWriteLine!("Need to allocate a new PDT");
                let addr = self.bdh.allocate(size_of::<PageDirectoryTable>(), 0x1000)
                    as *mut PageDirectoryTable;
                loggerWriteLine!("...and did that @ 0x{:X}", addr as usize);
                zeroMemory2(addr);
                pdt = addr;

                (*pdpt).setEntry(
                    pageDirectoryIndex,
                    pdt,
                    Execute::Yes,
                    Present::Yes,
                    Writable::Yes,
                    Cachable::No,
                    UserSupervisor::Supervisor,
                    WriteThrough::WriteTrough,
                );
            }

            let mut pt = (*pdt).getAddressForEntry(pageTableIndex);

            if pt as usize == 0 {
                loggerWriteLine!("Need to allocate a new PT...");
                let addr = self.bdh.allocate(size_of::<PageTable>(), 0x1000) as *mut PageTable;
                loggerWriteLine!("...and did that @ 0x{:X}", addr as usize);
                zeroMemory2(addr);
                pt = addr;

                (*pdt).setEntry(
                    pageTableIndex,
                    pt,
                    Execute::Yes,
                    Present::Yes,
                    Writable::Yes,
                    Cachable::No,
                    UserSupervisor::Supervisor,
                    WriteThrough::WriteTrough,
                );
            }

            for pageOffset in 0..numberOfPages {
                (*pt).setEntry(
                    pageIndex + pageOffset,
                    (startAddress + (pageOffset * SIZE_OF_PAGE)) as u64,
                    Execute::Yes,
                    Present::Yes,
                    Writable::Yes,
                    Cachable::No,
                    UserSupervisor::Supervisor,
                    WriteThrough::WriteTrough,
                );
            }
        }
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
