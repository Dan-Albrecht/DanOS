use crate::{
    acpi::{descriptionTable::DescriptionTable, fadt::FADT, mcfg::MCFG}, alignment::Aligned16, assemblyStuff::halt::haltLoop, vgaWrite, vgaWriteLine
};
use core::{fmt::Write, mem::size_of, ptr::addr_of};

// https://uefi.org/specs/ACPI/6.5/05_ACPI_Software_Programming_Model.html#root-system-description-table-rsdt
#[repr(C, packed)]
pub struct RsdtImpl {
    Signature: [u8; 4],
    Length: u32,
    Revision: u8,
    Checksum: u8,
    OEMID: [u8; 6],
    OemTableID: [u8; 8],
    OemRevision: [u8; 4],
    CreateID: [u8; 4],
    CreatorRevision: [u8; 4],
    FirstEntry: u32,
}

// BUGBUG: This one might not be aligned
// Root System Description Table
pub type RSDT = Aligned16<RsdtImpl>;

impl RSDT {
    pub fn walkEntries(&self) {
        let length = self.Field.Length as usize;
        let extraLength = length - size_of::<RsdtImpl>();
        let remainder = extraLength % 4;
        if remainder != 0 {
            vgaWriteLine!("Remaining space is not a multiple of 4");
            haltLoop();
        }

        let mut totalEntries = extraLength / 4;

        // You get one entry for free in the size of the struct
        totalEntries = totalEntries + 1;
        vgaWriteLine!(
            "Length of {} implies there's {} toal entries",
            length,
            totalEntries
        );

        let firstEntryAddress = addr_of!(self.Field.FirstEntry);

        for x in 0..totalEntries {
            let address = firstEntryAddress as usize + x * size_of::<u32>();
            
            unsafe {
                let ptr = address as *const u32;
                vgaWrite!("Entry {} @ 0x{:X} is a ", x, *ptr);
                let ptr = *ptr as *const DescriptionTable;
                (*ptr).printSignature();

                if &(*ptr).Signature == b"FACP" {
                    let ptr = ptr as *const FADT;
                    (*ptr).printSomeInfo();
                } else if &(*ptr).Signature == b"MCFG" {
                    let ptr = ptr as *const MCFG;
                    (*ptr).printSomeInfo();
                }
            }
        }
    }
}
