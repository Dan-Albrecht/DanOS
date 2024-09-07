use core::{fmt::Write, str::from_utf8};

use kernel_shared::assemblyStuff::halt::haltLoop;

use crate::loggerWriteLine;

use super::{pciGeneralDevice::PciGeneralDevice, rsdt::RSDT};

// https://uefi.org/specs/ACPI/6.5/05_ACPI_Software_Programming_Model.html#root-system-description-pointer-rsdp-structure
// Version 1 (Revsion 0) defintion
// Root System Description Pointer
#[repr(C, packed)]
pub struct RSDP {
    Signature: [u8; 8],
    Checksum: u8,
    OEMID: [u8; 6],
    Revision: u8,
    RsdtAddress: u32,
}

pub fn getRsdp() -> Option<*const PciGeneralDevice> {
    // We're going to assume this won't appear in the Extended BIOS Data Area (EBDA)
    // and just search the BIOS readonly area.
    let mut address: usize = 0x0E0000;
    loop {
        let ptr = address as *const RSDP;
        let checkMe = checkSignature(ptr);
        if let Ok(xxx) = checkMe {
            return xxx;
        }

        address = address + 16;
        if address > 0x0FFFFF {
            loggerWriteLine!("Didn't find RSDP. Halting.");
            haltLoop();
        }
    }
}

fn checkSignature(ptr: *const RSDP) -> Result<Option<*const PciGeneralDevice>, u8> {
    let expected = *b"RSD PTR ";
    unsafe {
        let toCheck = (*ptr).Signature;

        if toCheck == expected {
            loggerWriteLine!("Potential ACPI info at: 0x{:X}", ptr as usize);

            let mut calculated: u8 = 0;
            let asBytes = ptr as *const u8;
            for index in 0..20 {
                let byte = *asBytes.offset(index);
                calculated = calculated.wrapping_add(byte);
            }

            if calculated != 0 {
                loggerWriteLine!("Checksum fail (should be 0): {calculated}");
                return Err(1);
            }

            match from_utf8(&(*ptr).OEMID) {
                Ok(theString) => {
                    loggerWriteLine!("ACPI by {}", theString);

                    // Spec says this is always a 32 bit address
                    loggerWriteLine!("RSDT is at 0x{:X}", (*ptr).RsdtAddress as u32);
                }
                _ => {
                    loggerWriteLine!("Couldn't read ACPI OEM: {:?}", (*ptr).OEMID);
                }
            };

            // BUGBUG: Delete after debugging
            let rsdt = (*ptr).RsdtAddress as *const RSDT;
            let result = (*rsdt).walkEntries();
            return Ok(result);
        }
    }

    return Err(2);
}
