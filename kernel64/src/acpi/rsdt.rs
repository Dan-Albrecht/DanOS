use kernel_shared::assemblyStuff::halt::haltLoop;

use crate::{
    acpi::{descriptionTable::DescriptionTable, fadt::FADT, mcfg::MCFG}, loggerWrite, loggerWriteLine
};
use core::{fmt::Write, mem::size_of, ptr::{addr_of, read_unaligned}};

use super::pciGeneralDevice::PciGeneralDevice;

// https://uefi.org/specs/ACPI/6.5/05_ACPI_Software_Programming_Model.html#root-system-description-table-rsdt
// Root System Description Table
#[repr(C, packed)]
pub struct RSDT {
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

impl RSDT {
    pub fn walkEntries(&self)-> Option<*const PciGeneralDevice> {
        let length = self.Length as usize;
        let extraLength = length - size_of::<RSDT>();
        let remainder = extraLength % 4;
        if remainder != 0 {
            loggerWriteLine!("Remaining space is not a multiple of 4");
            haltLoop();
        }

        let firstEntryAddress = addr_of!(self.FirstEntry);
        let mut totalEntries = extraLength / 4;

        // You get one entry for free in the size of the struct
        totalEntries = totalEntries + 1;
        loggerWriteLine!(
            "Length of {} implies there's {} toal entries, pointer to first is: 0x{:X}",
            length,
            totalEntries,
            firstEntryAddress as usize,
        );

        let mut result = None;

        for x in 0..totalEntries {
            let address = firstEntryAddress as usize + x * size_of::<u32>();
            
            unsafe {
                let ptr = address as *const u32;
                let dtAddress = read_unaligned(ptr);
                loggerWrite!("Entry {} @ 0x{:X} is a ", x, dtAddress);
                let ptr = dtAddress as *const DescriptionTable;
                (*ptr).printSignature();

                if &(*ptr).Signature == b"FACP" {
                    let ptr = ptr as *const FADT;
                    (*ptr).printSomeInfo();
                } else if &(*ptr).Signature == b"MCFG" {
                    let ptr = ptr as *const MCFG;
                    let maybeResult = (*ptr).printSomeInfo();
                    if result == None && maybeResult != None {
                        result = maybeResult;
                    }
                }
            }
        }

        return  result;
    }
}
