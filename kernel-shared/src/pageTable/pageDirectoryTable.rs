use super::pageTable::PageTable;

#[repr(C, packed)]
pub struct PageDirectoryTable {
    // PDE
    pub Entry: *mut PageTable,
}