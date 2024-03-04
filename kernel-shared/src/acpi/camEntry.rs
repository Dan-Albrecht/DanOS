use crate::{assemblyStuff::halt::haltLoop, vgaWriteLine};

use super::mcfgEntry::McfgEntry;
use core::fmt::Write;

#[repr(C, packed)]
pub struct CamEntry {
    VendorID: u16,
    DeviceID: u16,
}

impl CamEntry {
    pub fn tryGetEntry(entry: &McfgEntry, bus: u8, device: u8, function: u8) -> Option<*const CamEntry> {
        let targetAddress = Self::calculateAddress(entry, bus, device, function);

        unsafe {
            let data = *(targetAddress as *const u32);
            if data == 0xFFFFFFFF {
                return None;
            } else{
                return Some(targetAddress as *const CamEntry);
            }
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
