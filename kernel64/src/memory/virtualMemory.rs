use core::fmt::Write;
use kernel_shared::{
    assemblyStuff::halt::haltLoop,
    haltLoopWithMessage,
    magicConstants::{
        PAGES_PER_TABLE, SIZE_OF_PAGE, SIZE_OF_PAGE_DIRECTORY, SIZE_OF_PAGE_DIRECTORY_POINTER,
        SIZE_OF_PAGE_TABLE,
    },
    memoryHelpers::{alignDown, haltOnMisaligned, zeroMemory2},
    pageTable::{pageBook::PageBook, pageDirectoryTable::PageDirectoryTable, pageTable::PageTable}, physicalMemory::{PhysicalMemoryManager, WhatDo},
};

use crate::loggerWriteLine;

use super::dumbHeap::BootstrapDumbHeap;

pub struct VirtualMemoryManager {
    physical: PhysicalMemoryManager,
    pageBook: PageBook,
    bdh: BootstrapDumbHeap,
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

    fn get_page_table_indexes(
        address: usize,
    )  {
        if !Self::is_canonical_address(address) {
            haltLoopWithMessage!("0x{:X} is not canonical", address);
        }

        let pml4_index = ((address >> 39) & 0x1FF) as usize;
        let pdpt_index = ((address >> 30) & 0x1FF) as usize;
        let pd_index = ((address >> 21) & 0x1FF) as usize;
        let pt_index = ((address >> 12) & 0x1FF) as usize;

        loggerWriteLine!("--> {} {} {} {}", pml4_index, pdpt_index, pd_index, pt_index);
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

        Self::get_page_table_indexes(startAddress);

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
                let addr =
                    self.bdh.allocate(size_of::<PageDirectoryTable>()) as *mut PageDirectoryTable;
                loggerWriteLine!("...and did that @ 0x{:X}", addr as usize);
                zeroMemory2(addr);
                pdt = addr;

                // BUGBUG: Figure out cachable story
                (*pdpt).setEntry(pageDirectoryIndex, pdt, true, false, false);
            }

            let mut pt = (*pdt).getAddressForEntry(pageTableIndex);

            if pt as usize == 0 {
                loggerWriteLine!("Need to allocate a new PT...");
                let addr = self.bdh.allocate(size_of::<PageTable>()) as *mut PageTable;
                loggerWriteLine!("...and did that @ 0x{:X}", addr as usize);
                zeroMemory2(addr);
                pt = addr;

                // BUGBUG: Figure out cachable story
                (*pdt).setEntry(pageTableIndex, pt, true, false, false);
            }

            // BUGBUG: Figure out cachable story
            (*pt).setEntry(pageIndex, numberOfPages, startAddress, true, true, false);
        }
    }
}
