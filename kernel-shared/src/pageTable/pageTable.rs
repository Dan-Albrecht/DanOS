use crate::{
    magicConstants::{ENTRIES_PER_PAGE_TABLE, SIZE_OF_PAGE},
    memoryHelpers::setCommonBitAndValidate, vgaWriteLine,
};

use core::fmt::Write;
use super::physicalPage::PhysicalPage;

#[repr(C, packed)]
pub struct PageTable {
    // PTE
    // The actual physical page address with some extra metadata bits or'd in
    Entries: [u64; ENTRIES_PER_PAGE_TABLE],
}

impl PageTable {
    pub fn setEntry(
        &mut self,
        startIndex: usize,
        numberOfPages: usize,
        startAddress: usize,
        present: bool,
        writable: bool,
        cachable: bool,
    ) {

        for relativeIndex in 0..numberOfPages {
            let actualIndex = relativeIndex + startIndex;
            let actualAddress = startAddress + (relativeIndex * SIZE_OF_PAGE);
            let entry = setCommonBitAndValidate("PTE", actualAddress, present, writable, cachable);
            self.Entries[actualIndex] = entry;
        }
    }

    pub fn getAddressForEntry(&self, index: usize) -> *const PhysicalPage {
        let mut entry = self.Entries[index];
        entry = entry & 0xF_FFFF_FFFF_F000;

        return entry as *const PhysicalPage;
    }
}
