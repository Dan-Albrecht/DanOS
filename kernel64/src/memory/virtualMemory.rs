use core::fmt::Write;
use kernel_shared::{
    assemblyStuff::halt::haltLoop,
    magicConstants::{
        FOURTH_PAGE_TABLE_LOCATION, SECOND_PAGE_TABLE_LOCATION, SIZE_OF_PAGE, SIZE_OF_PAGE_DIRECTORY, SIZE_OF_PAGE_TABLE, THIRD_PAGE_TABLE_LOCATION
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

impl VirtualMemoryManager {
    pub fn new(physical: *mut PhysicalMemoryManager, pageBook: PageBook) -> Self {
        VirtualMemoryManager { pageBook, physical }
    }

    pub fn identityMap(&self, requestedAddress: usize, allowReserved: bool) {
        let startAddress = alignDown(requestedAddress, SIZE_OF_PAGE);
        unsafe {
            (*self.physical).Reserve(startAddress, SIZE_OF_PAGE, allowReserved);
        }

        let pageDirectoryPointerIndex = 0;
        let pageDirectoryIndex = startAddress / SIZE_OF_PAGE_DIRECTORY;
        let pageTableIndex = (startAddress % SIZE_OF_PAGE_DIRECTORY) / SIZE_OF_PAGE_TABLE;

        vgaWriteLine!(
            "Requested 0x{:X} will use 0x{:X} and put it at 0x{:X},0x{:X},0x{:X}",
            requestedAddress,
            startAddress,
            pageDirectoryPointerIndex,
            pageDirectoryIndex,
            pageTableIndex,
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
        } else if requestedAddress == 0xFEBD_500C {
            pt = FOURTH_PAGE_TABLE_LOCATION as *mut PageTable;
        } else {
            vgaWriteLine!("Don't know how to 0x{:X}", requestedAddress);
            haltLoop();
        }

        unsafe {
            PageBook::initNewPageTable(pt, startAddress);
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
