use core::mem::size_of;

use crate::{
    magicConstants::{PHYSICAL_ADDRESS_OF_PAGE_BOOK, PHYSICAL_ADDRESS_VIRTUAL_MEMORY_START},
    memoryHelpers::zeroMemory2,
};

use super::{
    pageDirectoryPointerTable::PageDirectoryPointerTable, pageDirectoryTable::PageDirectoryTable,
    pageMapLevel4Table::PageMapLevel4Table, pageTable::PageTable, physicalPage::PhysicalPage
};

// This is the top of the hiearchy. Would have called this ThePageTable,
// but we already have a PageTable type much lower in the hierarchy.
pub struct PageBook {
    // This could be a PML5 if we ever wanted to support the extra bits of addressing
    pub Entry: *mut PageMapLevel4Table,
}

impl PageBook {
    // This will create and initalize
    pub fn fromScratch() -> *const PageBook {
        unsafe {
            let page = PHYSICAL_ADDRESS_OF_PAGE_BOOK as *mut PhysicalPage;
            zeroMemory2(page);
            
            let pt = (page as usize + size_of::<PhysicalPage>()) as *mut PageTable;
            // BUGBUG: Setting this uncachable for now as we're going to map the hard drive in this space
            // Need to get it, its own page
            (*pt).setEntry(0, page as u64, true, true, false);

            let pdt = (pt as usize + size_of::<PageTable>()) as *mut PageDirectoryTable;
            (*pdt).Entry = pt;

            let pdpt =
                (pdt as usize + size_of::<PageDirectoryTable>()) as *mut PageDirectoryPointerTable;
            (*pdpt).Entry = pdt;

            let pml4 =
                (pdpt as usize + size_of::<PageDirectoryPointerTable>()) as *mut PageMapLevel4Table;
            (*pml4).Entry = pdpt;

            let pageBook = (pml4 as usize + size_of::<PageMapLevel4Table>()) as *mut PageBook;
            (*pageBook).Entry = pml4;

            return pageBook;
        }
    }

    // This will just blindly assume you've already created this
    pub fn fromExisting() -> *const PageBook {
        todo!()
    }
}
