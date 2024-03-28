use super::pageDirectoryTable::PageDirectoryTable;

#[repr(C, packed)]
pub struct PageDirectoryPointerTable {
    // PDPE
    pub Entry: *mut PageDirectoryTable,
}
