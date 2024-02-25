use core::fmt::Write;
use crate::{alignment::Aligned16, assemblyStuff::halt::haltLoop, vgaWriteLine };

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
            // BUGBUG: Validate checksum
            vgaWriteLine!("OEM {:?}", (*ptr).Field.OEMID);
            return true;
        }
    }

    return false;
}
