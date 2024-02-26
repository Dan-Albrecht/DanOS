use crate::{alignment::Aligned16, assemblyStuff::halt::haltLoop, vgaWriteLine};
use core::{fmt::Write, str::from_utf8};

use super::rsdt::RSDT;

// Version 1 (Revsion 0) defintion
#[repr(C, packed)]
pub struct RsdpImpl {
    Signature: [u8; 8],
    Checksum: u8,
    OEMID: [u8; 6],
    Revision: u8,
    RsdtAddress: u32,
}

// Root System Description Pointer
pub type RSDP = Aligned16<RsdpImpl>;

impl RSDP {
    pub fn getRsdt(&self) -> *const RSDT {
        // BUGBUG: The theory here is this will properly 0-extend on 64 bit...
        self.Field.RsdtAddress as *const RSDT
    }
}

pub fn getRsdp() -> *const RSDP {
    // We're going to assume this won't appear in the Extended BIOS Data Area (EBDA)
    // and just search the BIOS readonly area.
    let mut address: usize = 0x0E0000;
    loop {
        let ptr = address as *const RSDP;
        if checkSignature(ptr) {
            return ptr;
        }

        address = address + 16;
        if address > 0x0FFFFF {
            vgaWriteLine!("Didn't find RSDP. Halting.");
            haltLoop();
        }
    }
}

fn checkSignature(ptr: *const RSDP) -> bool {
    let expected = *b"RSD PTR ";
    unsafe {
        let toCheck = (*ptr).Field.Signature;
        if toCheck == expected {

            vgaWriteLine!("Potential ACIP info at: 0x{:X}", ptr as usize);
            // BUGBUG: Validate checksum

            match from_utf8(&(*ptr).Field.OEMID) {
                Ok(theString) => {
                    vgaWriteLine!("ACPI by {}", theString);

                    // Spec says this is always a 32 bit address
                    vgaWriteLine!("RSDT is at 0x{:X}", (*ptr).Field.RsdtAddress as u32);
                }
                _ => {
                    vgaWriteLine!("Couldn't read ACPI OEM: {:?}", (*ptr).Field.OEMID);
                }
            };

            // BUGBUG: Delete after debugging
            (*(*ptr).getRsdt()).walkEntries();
            return true;
        }
    }

    return false;
}
