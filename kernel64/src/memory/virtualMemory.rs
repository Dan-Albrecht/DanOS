use core::fmt::Write;
use kernel_shared::{
    assemblyStuff::halt::haltLoop, haltLoopWithMessage, magicConstants::{
        FOURTH_PAGE_TABLE_LOCATION, PAGES_PER_TABLE, SECOND_PAGE_TABLE_LOCATION, SIZE_OF_PAGE, SIZE_OF_PAGE_DIRECTORY, SIZE_OF_PAGE_DIRECTORY_POINTER, SIZE_OF_PAGE_TABLE, THIRD_PAGE_TABLE_LOCATION
    }, memoryHelpers::{alignDown, alignUp, zeroMemory2}, pageTable::{pageBook::PageBook, pageDirectoryTable::PageDirectoryTable, pageTable::PageTable}, vgaWriteLine
};

use super::physicalMemory::PhysicalMemoryManager;

pub struct VirtualMemoryManager {
    physical: *mut PhysicalMemoryManager,
    pageBook: PageBook,
}

// BUGUBG: Come up with a better name
pub enum WhatDo {
    Normal,
    UseReserved,
    YoLo, // Allocate even if it isn't in the map. Seeing this for hardware IO.
}

impl VirtualMemoryManager {
    pub fn new(physical: *mut PhysicalMemoryManager, pageBook: PageBook) -> Self {
        VirtualMemoryManager { pageBook, physical }
    }

    pub fn identityMap(&self, requestedAddress: usize, numberOfPages: usize, whatDo: WhatDo) {
        // BUGBUG: Because we're rounding down, it is possible we won't end up mapping all memory the request in
        // we either need to have code to increase numberOfPages, or just rejected non-aligned requests
        let startAddress = alignDown(requestedAddress, SIZE_OF_PAGE);
        unsafe {
            (*self.physical).Reserve(startAddress, SIZE_OF_PAGE, whatDo);
        }

        let pageDirectoryPointerIndex = startAddress / SIZE_OF_PAGE_DIRECTORY_POINTER;
        let pageDirectoryIndex =
            (startAddress % SIZE_OF_PAGE_DIRECTORY_POINTER) / SIZE_OF_PAGE_DIRECTORY;
        let pageTableIndex = (startAddress % SIZE_OF_PAGE_DIRECTORY) / SIZE_OF_PAGE_TABLE;
        let pageIndex = (startAddress % SIZE_OF_PAGE_TABLE) / SIZE_OF_PAGE;

        if pageIndex + numberOfPages > PAGES_PER_TABLE {
            // BUGUBG: Handle it
            haltLoopWithMessage!("Request crosses page directoris and we cannot handle that yet");
        }

        vgaWriteLine!(
            "Requested 0x{:X} will start at 0x{:X} and live at {}, {}, {}, {}",
            requestedAddress,
            startAddress,
            pageDirectoryPointerIndex,
            pageDirectoryIndex,
            pageTableIndex,
            pageIndex,
        );

        if pageDirectoryPointerIndex != 0 {
            vgaWriteLine!(
                "Don't know how to do PDPT 0x{:X}",
                pageDirectoryPointerIndex
            );
            haltLoop();
        }

        // BUGUBG: Allocate this
        let pt: *mut PageTable;

        if requestedAddress == 0x7E0_0000 {
            pt = SECOND_PAGE_TABLE_LOCATION as *mut PageTable;
        } else if requestedAddress == 0xB000_0000 {
            pt = THIRD_PAGE_TABLE_LOCATION as *mut PageTable;
        } else if requestedAddress == 0xFEBD_500C || requestedAddress == 0xFEA0_0000 {
            pt = FOURTH_PAGE_TABLE_LOCATION as *mut PageTable;
        } else {
            vgaWriteLine!("Hardcode more tables for 0x{:X}", requestedAddress);
            haltLoop();
        }

        unsafe {
            // BUGBUG: Page table could already exist and we need to modify
            PageBook::initNewPageTable(pt, startAddress, pageIndex, numberOfPages);
            let pml4 = self.pageBook.getEntry();
            let pdpt = (*pml4).getAddressForEntry(pageDirectoryPointerIndex);
            let pdt: *mut PageDirectoryTable;

            if pageDirectoryIndex == 0 {
                // Can get existing
                pdt = (*pdpt).getAddressForEntry(pageDirectoryIndex);
            } else {
                let pdtAddress = pt as usize + size_of::<PageTable>();
                pdt = alignUp(pdtAddress, 0x1000) as *mut PageDirectoryTable;
                zeroMemory2(pdt);
                (*pdpt).setEntry(pageDirectoryIndex, pdt, true, true, false);
            }

            (*pdt).setEntry(pageTableIndex, pt, true, true, false);
        }
    }
}
