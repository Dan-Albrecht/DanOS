use core::str::from_utf8;
use core::fmt::Write;
use crate::vgaWriteLine;


#[repr(C, packed)]
pub struct DescriptionTable {
    Signature: [u8; 4],
    Length: u32,
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
                vgaWriteLine!("{}", theString);
            }
            _ => {
                vgaWriteLine!("Couldn't read signature: {:?}", self.Signature);
            }
        };
    }
}