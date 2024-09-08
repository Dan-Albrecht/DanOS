pub const SATA_DRIVE_BASE_CMD_BASE_ADDRESS: u32 = 0x50_0000;
pub const SATA_DRIVE_BASE_FIS_BASE_ADDRESS: u32 = 0x60_0000;
pub const SATA_DRIVE_BASE_COMMAND_TABLE_BASE_ADDRESS: usize = 0x70_0000;

pub const PHYSICAL_ADDRESS_OF_PAGE_BOOK: usize = 0x1000;
pub const PHYSICAL_ADDRESS_VIRTUAL_MEMORY_START: usize = 0x3000;

// BUGBUG: This need to agree with STAGE_1_5_LOAD_TARGET
pub static MEMORY_MAP_LOCATION: usize = 0x1000;

// BUGBUG: We should just do this on the stack, but this simplifies passing off from 32-bit to 64-bit for now
// We'll also verify in the build scripts we won't overrun this area
// This does mean XXX needs to keep in sync with the lowest of these addresses
pub const FIRST_PML4: usize = 0x1F_C000;
pub const FIRST_PDPT: usize = 0x1F_D000;
pub const FIRST_PD: usize = 0x1F_E000;
pub const FIRST_PT: usize = 0x1F_F000;

pub const DUMB_HEAP: usize = 0x1F_0000;
pub const DUMB_HEAP_LENGTH: usize = 0xC000;

pub static IDT_ADDRESS: usize = 0x9_0000;
pub static GDT_ADDRESS: usize = 0x8_0000;

// BUGBUG: These, especially the last two should be 'address range of' or something like that, the objects themselves are way smaller
pub static SIZE_OF_PAGE: usize = 0x1000;
pub static SIZE_OF_PAGE_TABLE: usize = 0x20_0000;
pub static SIZE_OF_PAGE_DIRECTORY: usize = 0x4000_0000;

// BUGBUG: Can we compile time ensure this is mod-0?
pub static PAGES_PER_TABLE: usize = SIZE_OF_PAGE_TABLE / SIZE_OF_PAGE;

pub const ENTRIES_PER_PAGE_TABLE: usize = 512;

#[cfg(target_pointer_width = "64")]
pub static SIZE_OF_PAGE_DIRECTORY_POINTER: usize = 0x80_0000_0000;
