use crate::memoryHelpers::setCommonBitAndValidate;

use super::pageTable::PageTable;

#[repr(C, packed)]
pub struct PageDirectoryTable {
    // PDE
    Entries: [u64; 512],
}

impl PageDirectoryTable {
    pub fn setEntry(
        &mut self,
        index: usize,
        entry: *const PageTable,
        present: bool,
        writable: bool,
        cachable: bool,
    ) {
        let address = setCommonBitAndValidate("PDE", entry as usize, present, writable, cachable);

        self.Entries[index] = address;
    }

    pub fn getAddressForEntry(&self, index: usize) -> *mut PageTable {
        let mut entry = self.Entries[index];
        entry = entry & 0xF_FFFF_FFFF_F000;

        return entry as *mut PageTable;
    }
}
