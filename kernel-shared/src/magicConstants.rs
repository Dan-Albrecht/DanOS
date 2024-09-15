pub const SATA_DRIVE_BASE_CMD_BASE_ADDRESS: u32 = 0x50_0000;
pub const SATA_DRIVE_BASE_FIS_BASE_ADDRESS: u32 = 0x60_0000;
pub const SATA_DRIVE_BASE_COMMAND_TABLE_BASE_ADDRESS: usize = 0x70_0000;

// BUGBUG: These, especially the last two should be 'address range of' or something like that, the objects themselves are way smaller
pub static SIZE_OF_PAGE: usize = 0x1000;
pub static SIZE_OF_PAGE_TABLE: usize = 0x20_0000;
pub static SIZE_OF_PAGE_DIRECTORY: usize = 0x4000_0000;

// BUGBUG: Can we compile time ensure this is mod-0?
pub static PAGES_PER_TABLE: usize = SIZE_OF_PAGE_TABLE / SIZE_OF_PAGE;

#[cfg(target_pointer_width = "64")]
pub static SIZE_OF_PAGE_DIRECTORY_POINTER: usize = 0x80_0000_0000;
