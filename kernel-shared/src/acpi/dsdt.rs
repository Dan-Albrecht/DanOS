use core::{fmt::Write, mem::size_of, ptr::addr_of};
use crate::vgaWriteLine;

//https://uefi.org/specs/ACPI/6.5/05_ACPI_Software_Programming_Model.html#differentiated-system-description-table-dsdt
// Differentiated System Description Table
#[repr(C, packed)]
pub struct DSDT {
    Signature: [u8; 4], // Suposed to be DSDT
    Length: u32,
    Revision: u8,
    Checksum: u8, // Whole struct should sum to 0
    OEMID: [u8; 6],
    OemTableID: [u8; 8],
    OemRevision: [u8; 4],
    CreateID: [u8; 4],
    CreatorRevision: [u8; 4],
    DefintionBlock: u8, // First byte of the block, Length tells us how long to go
}

impl DSDT {
    pub fn printSomething(&self) {
        let length = self.Length as usize;
        let blockLength = length - size_of::<DSDT>() + 1; // +1 because the first byte is defined in the struct
        let startAt = addr_of!(self.DefintionBlock);
        vgaWriteLine!("    Think we have {} bytes to read starting at 0x{:X}", blockLength, startAt as usize);
    }
}
