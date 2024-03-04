use core::str::from_utf8;
use core::fmt::Write;
use crate::vgaWriteLine;

// https://uefi.org/specs/ACPI/6.5/05_ACPI_Software_Programming_Model.html#system-description-table-header
#[repr(C, packed)]
pub struct DescriptionTable {
    pub Signature: [u8; 4], // https://uefi.org/specs/ACPI/6.5/05_ACPI_Software_Programming_Model.html#description-header-signatures-for-tables-defined-by-acpi
    pub Length: u32,
    Revision: u8,
    Checksum: u8,
    OEMID: [u8; 6],
    OemTableID: [u8; 8],
    OemRevision: [u8; 4],
    CreateID: [u8; 4],
    CreatorRevision: [u8; 4],
}

impl DescriptionTable {
    pub fn printSignature(&self) {
        match from_utf8(&self.Signature) {
            Ok(theString) => {
                let length = self.Length;
                vgaWriteLine!("{} byte long {}", length, theString);
            }
            _ => {
                vgaWriteLine!("Couldn't read signature: {:?}", self.Signature);
            }
        };
    }
}