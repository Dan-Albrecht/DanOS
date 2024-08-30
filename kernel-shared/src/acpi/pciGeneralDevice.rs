use crate::{assemblyStuff::halt::haltLoop, vgaWrite, vgaWriteLine};
use core::{fmt::Write, ptr::{addr_of, read_volatile}};

use super::{
    bar::Bar,
    pciCommonHeader::{PciCommonHeader, PciHeaderType},
};

// https://wiki.osdev.org/PCI#Header_Type_0x0
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
            PciHeaderType::General | PciHeaderType::MultiFunctionGeneral => {
                return Some(commonHeader as *const _ as *const PciGeneralDevice)
            }
            _ => return None,
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

    pub fn tryGetBarAddress(&self, barNumber: u8) -> Option<Bar> {
        if barNumber >= 6 {
            return None;
        }

        let barAddress = match barNumber {
            0 => addr_of!(self.BAR0),
            1 => addr_of!(self.BAR1),
            2 => addr_of!(self.BAR2),
            3 => addr_of!(self.BAR3),
            4 => addr_of!(self.BAR4),
            5 => addr_of!(self.BAR5),
            _ => return None,
        };

        let barAddress = barAddress as *mut u32;

        unsafe {
            let barValue = read_volatile(barAddress);
            if barValue == 0 {
                return None;
            }

            if barValue & 1 == 1 {
                // I/O port
                return None;
            }

            let memoryType = (barValue >> 1) & 0x3;
            if memoryType != 0 {
                // BUGBUG: Only supporting 32-bit for now
                vgaWriteLine!("Don't know how to handle memory type {}", memoryType);
                haltLoop();
            }

            let address = barValue & 0xFFFFFFF0;
            let result = Bar::new(address, barValue, barAddress);

            return Some(result);
        }
    }

    fn printBarDetails(barNumber: u8, barValue: u32) {
        if barValue != 0 {
            vgaWrite!("      BAR{}: 0x{:X}", barNumber, barValue);
            if barValue & 1 == 1 {
                vgaWriteLine!(" I/O @ 0x{:X}", barValue & 0xFFFFFFFC);
            } else {
                let memoryType = (barValue >> 1) & 0x3;
                match memoryType {
                    0 => {
                        vgaWriteLine!(" 32-bit memory @ 0x{:X}", barValue & 0xFFFFFFF0);
                    }
                    1 => {
                        vgaWriteLine!(" (reserved-type)");
                    }
                    2 => {
                        vgaWriteLine!(" 64-bit memory; dunno how to handle that...");
                        haltLoop();
                    }
                    _ => {
                        vgaWriteLine!(" ({}-type memory)", memoryType);
                    }
                }
            }
        }
    }
}
