use crate::{vgaWrite, vgaWriteLine};
use core::fmt::Write;

use super::pciCommonHeader::{PciCommonHeader, PciHeaderType};

#[repr(C, packed)]
pub struct PciGeneralDevice {
    PciCommonHeader: PciCommonHeader,
    BAR0: u32, // Base Address Register
    BAR1: u32,
    BAR2: u32,
    BAR3: u32,
    BAR4: u32,
    BAR5: u32,
    CardbusCIS: u32, // Cardbus Card Information Structure pointer
    SubsystemVendorID: u16,
    SubsystemID: u16,
    ExpansionAddress: u32, // Base address for expansion ROM
    CapabilitiesOffset: u8,
    Reserved: [u8; 7],
    InterruptLine: u8,
    InterruptPIN: u8,
    MinGrant: u8,
    MaxLatency: u8,
}

impl PciGeneralDevice {
    pub fn tryGet(commonHeader: &PciCommonHeader) -> Option<*const PciGeneralDevice> {
        let headerType = commonHeader.getType();
        match headerType {
            PciHeaderType::General | PciHeaderType::MultiFunctionGeneral => return Some(commonHeader as *const _ as *const PciGeneralDevice),
            _ => return  None,
        }
    }

    pub fn printBars(&self) {
        Self::printBarDetails(0, self.BAR0);
        Self::printBarDetails(1, self.BAR1);
        Self::printBarDetails(2, self.BAR2);
        Self::printBarDetails(3, self.BAR3);
        Self::printBarDetails(4, self.BAR4);
        Self::printBarDetails(5, self.BAR5);
    }

    fn printBarDetails(barNumber: u8, barValue: u32) {
        if barValue != 0 {
            vgaWrite!("      BAR{}: 0x{:X}", barNumber, barValue);
            if barValue & 1 == 1 {
                vgaWriteLine!(" I/O @ 0x{:X}", barValue & 0xFFFFFFFC);
            } else {
                let memoryType = (barValue >> 1) & 0x3;
                match memoryType {
                    0 => {vgaWriteLine!(" 32-bit memory @ 0x{:X}", barValue & 0xFFFFFFF0);},
                    1 => {vgaWriteLine!(" (reserved-type)");},
                    // BUGBUG: Implment fully, needs two entries to get full address
                    2 => {vgaWriteLine!(" 64-bit memory");},
                    _ => {vgaWriteLine!(" ({}-type memory)", memoryType);},
                }
            }
        }
    }
}
