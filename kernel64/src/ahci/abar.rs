use kernel_shared::assemblyStuff::halt::haltLoop;

use crate::{
    acpi::{bar::Bar, pciGeneralDevice::PciGeneralDevice}, loggerWriteLine, vgaWriteLine
};
use core::fmt::Write;

// AHCI Base Address Register
pub struct ABar {
    _Bar: Bar,
    pub HBA: *const HbaData,
}

// Host Bus Adaptor
// 3 HBA Memory Registers
#[repr(C, packed)]
pub struct HbaData {
    pub GHC: GenericHostControl,
    Reserved: [u8; 0x34],    // Normal Reserved
    Reserved2: [u8; 0x40],   // Reserved for NVMHCI 🤷🏻‍♂️
    Vendor: [u8; 0x60],      // Vendor specific registers
    pub Port0: PortRegister, // Port X control register. Consult with GHC.PI for find out which ones are actually avilable
    pub Port1: PortRegister,
    pub Port2: PortRegister,
    pub Port3: PortRegister,
    pub Port4: PortRegister,
    pub Port5: PortRegister,
    pub Port6: PortRegister,
    pub Port7: PortRegister,
    pub Port8: PortRegister,
    pub Port9: PortRegister,
    pub Port10: PortRegister,
    pub Port11: PortRegister,
    pub Port12: PortRegister,
    pub Port13: PortRegister,
    pub Port14: PortRegister,
    pub Port15: PortRegister,
    pub Port16: PortRegister,
    pub Port17: PortRegister,
    pub Port18: PortRegister,
    pub Port19: PortRegister,
    pub Port20: PortRegister,
    pub Port21: PortRegister,
    pub Port22: PortRegister,
    pub Port23: PortRegister,
    pub Port24: PortRegister,
    pub Port25: PortRegister,
    pub Port26: PortRegister,
    pub Port27: PortRegister,
    pub Port28: PortRegister,
    pub Port29: PortRegister,
    pub Port30: PortRegister,
    pub Port31: PortRegister,
}

// 3.1 Generic Host Control
#[repr(C, packed)]
pub struct GenericHostControl {
    Cap: u32,       // Host Capabilities
    Ghc: u32,       // Global Host Control
    IS: u32,        // Interrupt Status
    pub PI: u32,    // Ports Implemented
    VS: u32,        // Version
    Ccc_Ctl: u32,   // Command Completion Coalescing Control
    Ccc_Ports: u32, // Command Completion Coalescing Ports
    Em_Log: u32,    // Encolsure Managment Location
    Em_Ctl: u32,    // Enclosure Managment Control
    Cap2: u32,      // Host Capabilities Extended
    Bohc: u32,      // BIOS /OS Handoff Control & Status
}

// 3.3 Port Registers (one set per port)
#[repr(C, packed)]
pub struct PortRegister {
    pub CLB: u32,  // Command List Base Address
    pub CLBU: u32, // Command List Base Address Upper 32-Bits
    pub FB: u32,   // FIS Base Address
    pub FBU: u32,  // FIS Base Address Upper 32-Bits
    pub IS: u32,   // Interrupt Status
    pub IE: u32,   // Interrupt Enable
    pub CMD: u32,  // Command & Status
    Reserved: u32,
    pub TFD: u32,    // Task File Data
    pub SIG: u32,    // Signature
    pub SSTS: u32,   // Serial ATA Status (SCR0: SSatus)
    pub SCTL: u32,   // Serial ATA Control (SCR2: SControl)
    pub SERR: u32,   // Serial ATA Error (SCR1: SError)
    pub SACT: u32,   // Serial ATA Active (SCR3: SActive)
    pub CI: u32,     // Command Issue
    pub SNTF: u32,   // Serial ATA Notification (SCr4: SNotification)
    pub FSB: u32,    // FIS-based Switching Control
    pub DEVSLP: u32, // Device Setup
    Reserved2: [u8; 0x28],
    pub VS: u32, // Vendor Specific
}
impl PortRegister {
    pub fn setClb(&mut self, address: u32) {
        if address & 0b11_1111_1111 != 0 {
            loggerWriteLine!("0x{:X} is not 1K-byte aligned", address);
            haltLoop();
        }

        self.CLB = address;

        // In 32-bit space
        self.CLBU = 0;
    }
}

impl ABar {
    pub fn tryGet(device: &PciGeneralDevice) -> Option<ABar> {
        // Docs say BAR 5 is always the one we need
        if let Some(bar) = device.tryGetBarAddress(5) {
            let addr = bar.BarTarget as *const HbaData;
            loggerWriteLine!("Got BAR 5 @ 0x{:X}", addr as usize);
            return Some(ABar {
                _Bar: bar,
                HBA: addr as *const HbaData,
            });
        }

        return None;
    }
}
