use crate::memoryHelpers::setCommonBitAndValidate;

use super::physicalPage::PhysicalPage;

#[repr(C, packed)]
pub struct PageTable {
    // PTE
    // The actual physical page address with some extra metadata bits or'd in
    Entries: [u64; 512],
}

impl PageTable {
    pub fn setEntry(
        &mut self,
        index: usize,
        entry: *const PhysicalPage,
        present: bool,
        writable: bool,
        cachable: bool,
    ) {
        let address = setCommonBitAndValidate("PTE", entry as usize, present, writable, cachable);

        self.Entries[index] = address;
    }

    pub fn getAddressForEntry(&self, index: usize) -> *const PhysicalPage {
        let mut entry = self.Entries[index];
        entry = entry & 0xF_FFFF_FFFF_F000;

        return entry as *const PhysicalPage;
    }
}
