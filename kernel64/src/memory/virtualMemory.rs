use core::fmt::Write;
use kernel_shared::{
    assemblyStuff::halt::haltLoop,
    haltLoopWithMessage,
    magicConstants::{
        PAGES_PER_TABLE, SIZE_OF_PAGE, SIZE_OF_PAGE_DIRECTORY, SIZE_OF_PAGE_DIRECTORY_POINTER,
        SIZE_OF_PAGE_TABLE,
    },
    memoryHelpers::{alignDown, alignUp, zeroMemory2},
    pageTable::{pageBook::PageBook, pageDirectoryTable::PageDirectoryTable, pageTable::PageTable},
    vgaWriteLine,
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
        VirtualMemoryManager {
            pageBook: pageBook,
            physical: physical,
        }
    }

    pub fn identityMap(&mut self, requestedAddress: usize, numberOfPages: usize, whatDo: WhatDo) {
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

        // BUGUBG: Don't be lazy
        if pageDirectoryPointerIndex != 0 {
            vgaWriteLine!(
                "Don't know how to do PDPT 0x{:X}",
                pageDirectoryPointerIndex
            );
            haltLoop();
        }

        unsafe {
            let pml4 = self.pageBook.getEntry();
            let pdpt = (*pml4).getAddressForEntry(pageDirectoryPointerIndex);

            let pdt = (*pdpt).getAddressForEntry(pageDirectoryIndex);
            if pdt as usize == 0 {
                vgaWriteLine!("Need to allocate a new PDT");
                haltLoop();
            }

            let mut pt = (*pdt).getAddressForEntry(pageTableIndex);
            if pt as usize == 0 {
                vgaWriteLine!("Need to allocate a new PT");
                haltLoop();
            }

            // BUGBUG: Figure out cachable story
            (*pt).setEntry(pageIndex, startAddress, true, true, false);
        }
    }
}
