use crate::memoryHelpers::setCommonBitAndValidate;

use super::pageDirectoryPointerTable::PageDirectoryPointerTable;

#[repr(C, packed)]
pub struct PageMapLevel4Table {
    // PML4E
    Entries: [u64; 512],
}

impl PageMapLevel4Table {
    pub fn setEntry(
        &mut self,
        index: usize,
        entry: *const PageDirectoryPointerTable,
        present: bool,
        writable: bool,
        cachable: bool,
    ) {
        let address = setCommonBitAndValidate("PML4E", entry as usize, present, writable, cachable);

        self.Entries[index] = address;
    }

    pub fn getAddressForEntry(&self, index: usize) -> *const PageDirectoryPointerTable {
        let mut entry = self.Entries[index];
        entry = entry & 0xF_FFFF_FFFF_F000;

        return entry as *const PageDirectoryPointerTable;
    }
}
