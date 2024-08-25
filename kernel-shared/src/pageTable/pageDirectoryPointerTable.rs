use crate::memoryHelpers::setCommonBitAndValidate;

use super::pageDirectoryTable::PageDirectoryTable;

#[repr(C, packed)]
pub struct PageDirectoryPointerTable {
    // PDPE
    Entries: [u64; 512],
}

impl PageDirectoryPointerTable {
    pub fn setEntry(
        &mut self,
        index: usize,
        entry: *const PageDirectoryTable,
        present: bool,
        writable: bool,
        cachable: bool,
    ) {
        let address = setCommonBitAndValidate("PDPE", entry as usize, present, writable, cachable);

        self.Entries[index] = address;
    }

    pub fn getAddressForEntry(&self, index: usize) -> *mut PageDirectoryTable {
        let mut entry = self.Entries[index];
        entry = entry & 0xF_FFFF_FFFF_F000;

        return entry as *mut PageDirectoryTable;
    }
}
