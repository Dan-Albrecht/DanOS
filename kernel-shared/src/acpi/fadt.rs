use crate::{acpi::dsdt::DSDT, vgaWriteLine};
use core::{fmt::Write, ptr::addr_of};

// https://uefi.org/specs/ACPI/6.5/05_ACPI_Software_Programming_Model.html#fixed-acpi-description-table-fadt
// Fixed ACPI Description Table
#[repr(C, packed)]
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
    PM1a_EVT_BLK: u32,
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

        let pml = self.PM1a_EVT_BLK;
        let x = addr_of!(self.PM1a_EVT_BLK) as usize;
        vgaWriteLine!("  PM1a_EVT_BLK @ 0x{:X} says look to 0x{:X}", x, pml);
    }
}
