use crate::{acpi::dsdt::DSDT, vgaWriteLine};
use core::{fmt::Write, ptr::addr_of};

// https://uefi.org/specs/ACPI/6.5/05_ACPI_Software_Programming_Model.html#fixed-acpi-description-table-fadt
// Fixed ACPI Description Table
#[repr(C, packed)]
#[cfg(target_pointer_width = "32")]
pub struct FADT {
    Signature: [u8; 4], // Suposed to be FACP
    Length: u32,
    MajorVersion: u8,
    Checksum: u8, // Whole struct should sum to 0
    OEMID: [u8; 6],
    OemTableID: [u8; 8],
    OemRevision: [u8; 4],
    CreateID: [u8; 4],
    CreatorRevision: [u8; 4],
    FirmwareCtrl: u32,  // Physical memory address of the FACS
    DSDT: u32,          // Physical memory address of the DSDT
    Padding: [u8; 0xC], // Plus a bunch of other stuff I don't currently care about
    PM1a_EVT_BLK: u32,  //System port address of the PM1a Event Register Block
}

// So what we're trying to do is have the compiled version use the native pointer width,
// but still have access to the other value via explcitly requesting.
#[repr(C, packed)]
#[cfg(target_pointer_width = "64")]
pub struct FADT {
    Signature: [u8; 4], // Suposed to be FACP
    Length: u32,
    MajorVersion: u8,
    Checksum: u8, // Whole struct should sum to 0
    OEMID: [u8; 6],
    OemTableID: [u8; 8],
    OemRevision: [u8; 4],
    CreateID: [u8; 4],
    CreatorRevision: [u8; 4],
    FirmwareCtrl_32: u32,    // Physical memory address of the FACS
    DSDT_32: u32,            // Physical memory address of the DSDT
    Padding: [u8; 0xC],      // Plus a bunch of other stuff I don't currently care about
    PM1a_EVT_BLK_32: u32,    // System port address of the PM1a Event Register Block
    MorePadding: [u8; 0x48], // More stuff I don't care about
    FirmwareCtrl: u64,       // Extended physical address of the FACS.
    DSDT: u64,               // Extended physical address of the DSDT
    PM1a_EVT_BLK: [u8; 12],  //
}

impl FADT {
    pub fn printSomeInfo(&self) {
        let facs = self.FirmwareCtrl;
        let dsdt = self.DSDT;
        let length = self.Length;
        vgaWriteLine!(
            "  FADT is {} bytes long, FACS is @ 0x{:X}, DSDT is @ 0x{:X}",
            length,
            facs,
            dsdt
        );
        let ptr = dsdt as *const DSDT;
        unsafe {
            (*ptr).printSomething();
        }

        let pml = self.getPML1aBlock();
        let x = addr_of!(self.PM1a_EVT_BLK) as usize;
        vgaWriteLine!("  PM1a_EVT_BLK @ 0x{:X} says look to 0x{:X}", x, pml);
    }

    #[cfg(target_pointer_width = "32")]
    fn getPML1aBlock(&self) -> u32 {
        self.PM1a_EVT_BLK
    }

    #[cfg(target_pointer_width = "64")]
    fn getPML1aBlock(&self) -> u64 {
        // BUGBUG: Figure this thing out, below code is from ChatGPT and I haven't verified
        // https://uefi.org/specs/ACPI/6.5/04_ACPI_Hardware_Specification.html#pm1-event-grouping
        let address = u32::from_le_bytes(self.PM1a_EVT_BLK[0..4].try_into().unwrap());
        return  address as u64;
    }
}
