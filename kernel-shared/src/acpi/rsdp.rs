use crate::{alignment::Aligned16, assemblyStuff::halt::haltLoop, vgaWriteLine};
use core::{fmt::Write, str::from_utf8};

use super::rsdt::RSDT;

// https://uefi.org/specs/ACPI/6.5/05_ACPI_Software_Programming_Model.html#root-system-description-pointer-rsdp-structure
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

#[allow(arithmetic_overflow)]
fn checkSignature(ptr: *const RSDP) -> bool {
    let expected = *b"RSD PTR ";
    unsafe {
        let toCheck = (*ptr).Field.Signature;
        if toCheck == expected {
            vgaWriteLine!("Potential ACPI info at: 0x{:X}", ptr as usize);
            
            let mut calculated: u8 = 0;
            let asBytes = ptr as *const u8;
            for index in 0..20 {
                let byte = *asBytes.offset(index);
                calculated = calculated + byte;
            }

            if calculated != 0 {
                vgaWriteLine!("Checksum fail (should be 0): {calculated}");
                return false;
            }

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
