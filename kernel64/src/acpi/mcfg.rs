use core::{fmt::Write, mem::size_of, ptr::addr_of};

use kernel_shared::assemblyStuff::halt::haltLoop;

use crate::{acpi::mcfgEntry::McfgEntry, vgaWriteLine};

use super::pciGeneralDevice::PciGeneralDevice;

// UEFI doesn't doc this and links to something you need to register for, so OSDev it is
// https://wiki.osdev.org/PCI_Express#Enhanced_Configuration_Mechanism
// Memory-mapped ConFiGuration space
// Totally not sketchy site: https://www.ufoit.com/thread-7-1-1.html
#[repr(C, packed)]
pub struct MCFG {
    Signature: [u8; 4],
    Length: u32,
    Revision: u8,
    Checksum: u8,
    OEMID: [u8; 6],
    OemTableID: [u8; 8],
    OemRevision: [u8; 4],
    CreateID: [u8; 4],
    CreatorRevision: [u8; 4],
    Reserved: u64,
    FirstConfigEntry: u8,
}

impl MCFG {
    pub fn printSomeInfo(&self) -> Option<*const PciGeneralDevice>{
        let length = self.Length;
        let lengthForEntries = length - size_of::<MCFG>() as u32 + 1; // +1 as FirstConfigEntryis the first byte of the first entry, so shouldn't count as the base size of this table
        let size = size_of::<McfgEntry>() as u32;
        let numOfEntries = lengthForEntries / size;

        if lengthForEntries % size != 0 {
            vgaWriteLine!(
                "  Remaining length ({}) isn't divisible by {}",
                lengthForEntries,
                size
            );
            haltLoop();
        }

        let first = addr_of!(self.FirstConfigEntry) as usize;
        vgaWriteLine!(
            "  MCFG has {} entries, first is at 0x{:X}",
            numOfEntries,
            first
        );

        let base = addr_of!(self.FirstConfigEntry) as *const McfgEntry;
        let mut result = None;
        unsafe {
            for index in 0..numOfEntries as isize {
                let entry = base.offset(index);
                let maybeResult = (*entry).printSomeInfo();
                if result == None && maybeResult != None{
                    result = maybeResult;
                }
            }
        }

        return result;
    }
}
