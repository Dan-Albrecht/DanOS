use kernel_shared::assemblyStuff::halt::haltLoop;

use crate::vgaWriteLine;

use super::mcfgEntry::McfgEntry;
use core::fmt::Write;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum PciHeaderType {
    General,
    NormalBridge, // PCI to PCI
    CardBridge,   // PCI to CardBus
    MultiFunctionGeneral,
    Dunno,
}

#[repr(C, packed)]
pub struct PciCommonHeader {
    VendorID: u16,
    DeviceID: u16,
    Command: u16,
    Status: u16,
    RevisionID: u8,
    pub ProgIF: u8, // Programming Interface Byte
    pub Subclass: u8,
    pub ClassCode: u8,
    CacheLineSize: u8,
    LatenchTimer: u8,
    pub HeaderType: u8,
    BIST: u8, // Built In Self Test
}

impl PciCommonHeader {
    pub fn tryGetEntry(
        entry: &McfgEntry,
        bus: u8,
        device: u8,
        function: u8,
    ) -> Option<*const PciCommonHeader> {
        let targetAddress = Self::calculateAddress(entry, bus, device, function);

        unsafe {
            // Note if you look at this memory directly in Bochs it doesn't reflect what it actually is
            // You have to read it
            let data = *(targetAddress as *const u32);
            if data == 0xFFFFFFFF {
                return None;
            } else {
                return Some(targetAddress as *const PciCommonHeader);
            }
        }
    }

    pub fn getType(&self) -> PciHeaderType {
        match self.HeaderType {
            0 => PciHeaderType::General,
            1 => PciHeaderType::NormalBridge,
            2 => PciHeaderType::CardBridge,
            0x80 => PciHeaderType::MultiFunctionGeneral, // BUGBUG: I'm pretty sure this should be a bit flag on the other types
            _ => PciHeaderType::Dunno,
        }
    }

    fn calculateAddress(entry: &McfgEntry, bus: u8, device: u8, function: u8) -> usize {
        if device >= 32 {
            vgaWriteLine!("Device # {} >= 32", device);
            haltLoop();
        }

        if function >= 8 {
            vgaWriteLine!("Function # {} >= 8", function);
            haltLoop();
        }

        let base = entry.BaseAddress;

        if base > usize::MAX as u64 {
            vgaWriteLine!("    Base is too big for this platform");
            haltLoop();
        }

        let bigStart = entry.StartBus as u64;
        let bigBus = bus as u64;
        let bigDevice = device as u64;
        let bigFunction = function as u64;

        let a = (bigBus - bigStart) << 20;
        let b = bigDevice << 15;
        let c = bigFunction << 12;
        let d = a | b | c;
        let e = base + d;

        if e > usize::MAX as u64 {
            vgaWriteLine!("    Calculated is too big for this platform");
            haltLoop();
        }

        return e as usize;
    }
}
