use crate::{acpi::dsdt::DSDT, vgaWriteLine};
use core::fmt::Write;

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
    FirmwareCtrl: u32, // Physical memory address of the FACS
    DSDT: u32,         // Physical memory address of the DSDT
                       // Plus a bunch of other stuff I don't currently care about
}

impl FADT {
    pub fn printSomeInfo(&self) {
        let dsdt = self.DSDT;
        vgaWriteLine!("  Next Pointer: 0x{:X}", dsdt);
        let ptr = dsdt as *const DSDT;
        unsafe {
            (*ptr).printSomething();
        }
    }
}
