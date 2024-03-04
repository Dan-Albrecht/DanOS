use core::fmt::Write;

use crate::{acpi::camEntry::CamEntry, vgaWriteLine};

// Same no UEFI docs story as MCFG
// https://wiki.osdev.org/PCI_Express#Enhanced_Configuration_Mechanism
// Also useful https://wiki.qemu.org/images/f/f6/PCIvsPCIe.pdf
#[repr(C, packed)]
pub struct McfgEntry {
    pub BaseAddress: u64,
    pub SegmentGroup: u16,
    pub StartBus: u8,
    pub EndBus: u8,
    pub Reserved: u32,
}

impl McfgEntry {
    pub fn printSomeInfo(&self) {
        let base = self.BaseAddress;
        let seg = self.SegmentGroup;
        let start = self.StartBus;
        let end = self.EndBus;

        vgaWriteLine!(
            "    Base 0x{:X} for group {} busses {}..={}",
            base,
            seg,
            start,
            end
        );

        for device in 0..32 {
            match CamEntry::tryGetEntry(&self, 0, device, 0) {
                Some(_) => {
                    vgaWriteLine!("    Device {} exists", device);
                }
                None => (),
            }
        }
    }
}
