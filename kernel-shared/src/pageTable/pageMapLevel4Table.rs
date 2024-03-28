use super::pageDirectoryPointerTable::PageDirectoryPointerTable;

#[repr(C, packed)]
pub struct PageMapLevel4Table {
    // PML4E
    // BUGBUG: These should all be arrays/pointers down the hiearchy, but
    // this lets us get a singel entry we'll map for now
    pub Entry: *mut PageDirectoryPointerTable,
}
