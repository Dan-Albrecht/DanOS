use kernel_shared::assemblyStuff::halt::haltLoop;

use crate::{
    acpi::pciGeneralDevice::PciGeneralDevice, loggerWriteLine
};
use core::{
    fmt::Write,
    ptr::{addr_of, read_volatile},
};

use super::{abar::{ABar, PortRegister}, sataDrive::SataDrive};

pub struct Controller {
    ABar: ABar,
}

impl Controller {
    pub fn tryGet(header: *const PciGeneralDevice) -> Option<SataDrive> {
        unsafe {
            if let Some(abar) = ABar::tryGet(&*header) {
                let controller = Controller { ABar: abar };
                if let Some(port) = controller.enumeratePorts() {
                    let drive = SataDrive::new(controller, port);

                    return Some(drive);
                } else {
                    loggerWriteLine!("Didn't find a SATA port");
                    return None;
                }
            } else {
                loggerWriteLine!("ABar returned None");
                return None;
            }
        }
    }

    pub fn enumeratePorts(&self) -> Option<u8> {
        unsafe {
            let hba = read_volatile(self.ABar.HBA);
            let mut pi = hba.GHC.PI;
            for index in 0..32 {
                if (pi & 1) != 0 {
                    loggerWriteLine!("Something at port {index}");
                    if self.isSATA(index) == Some(true) {
                        return Some(index);
                    }
                    pi >>= 1;
                }
            }
        }

        return None;
    }

    pub fn getPort(&self, index: u8) -> *mut PortRegister {
        unsafe {
            let port = match index {
                0 => &(*(self.ABar.HBA)).Port0,
                1 => &(*(self.ABar.HBA)).Port1,
                2 => &(*(self.ABar.HBA)).Port2,
                3 => &(*(self.ABar.HBA)).Port3,
                4 => &(*(self.ABar.HBA)).Port4,
                5 => &(*(self.ABar.HBA)).Port5,
                6 => &(*(self.ABar.HBA)).Port6,
                7 => &(*(self.ABar.HBA)).Port7,
                8 => &(*(self.ABar.HBA)).Port8,
                9 => &(*(self.ABar.HBA)).Port9,
                10 => &(*(self.ABar.HBA)).Port10,
                11 => &(*(self.ABar.HBA)).Port11,
                12 => &(*(self.ABar.HBA)).Port12,
                13 => &(*(self.ABar.HBA)).Port13,
                14 => &(*(self.ABar.HBA)).Port14,
                15 => &(*(self.ABar.HBA)).Port15,
                16 => &(*(self.ABar.HBA)).Port16,
                17 => &(*(self.ABar.HBA)).Port17,
                18 => &(*(self.ABar.HBA)).Port18,
                19 => &(*(self.ABar.HBA)).Port19,
                20 => &(*(self.ABar.HBA)).Port20,
                21 => &(*(self.ABar.HBA)).Port21,
                22 => &(*(self.ABar.HBA)).Port22,
                23 => &(*(self.ABar.HBA)).Port23,
                24 => &(*(self.ABar.HBA)).Port24,
                25 => &(*(self.ABar.HBA)).Port25,
                26 => &(*(self.ABar.HBA)).Port26,
                27 => &(*(self.ABar.HBA)).Port27,
                28 => &(*(self.ABar.HBA)).Port28,
                29 => &(*(self.ABar.HBA)).Port29,
                30 => &(*(self.ABar.HBA)).Port30,
                31 => &(*(self.ABar.HBA)).Port31,
                _ => {
                    loggerWriteLine!("Port index {index} is bogus!");
                    haltLoop();
                }
            };

            // BUGBUG: We're making imutable, mutable here, is this allowed?
            return port as *const _ as *mut PortRegister;
        }
    }

    unsafe fn isSATA(&self, index: u8) -> Option<bool> {
        let port = self.getPort(index);

        // 3.3.10 Offset 28h: PxSSTS – Port x Serial ATA Status (SCR0: SStatus)
        // BUGBUG: Volatile shouldn't be neeed, remove after debugging
        let portAddr = port as *const _ as usize;
        let statusAddr = addr_of!((*port).SSTS);
        let status = read_volatile(statusAddr);
        loggerWriteLine!(
            "Port {} is @ 0x{:X}, status @ 0x{:X} with status 0x{:X}",
            index,
            portAddr as usize,
            statusAddr as usize,
            status
        );

        let ipm = Self::parseIPM((status >> 8) & 0xF);
        let det = Self::praseDET(status & 0xF);

        if det != DET::DetectedAndEstablished {
            loggerWriteLine!("DET is {:?} so can't use it", det);
            return None;
        }

        if ipm != IPM::Active {
            loggerWriteLine!("IPM is {:?} so can't use it", ipm);
            return None;
        }

        // 3.3.9 Offset 24h: PxSIG – Port x Signature
        // BUGBUG: OSDev is odd
        let sig = (*port).SIG;
        match sig {
            0x00000101 => {
                loggerWriteLine!("SATA");
                return Some(true);
            }
            0xEB140101 => {
                loggerWriteLine!("SATAPI");
            }
            0xC33C0101 => {
                loggerWriteLine!("Enclosure management bridge");
            }
            0x96690101 => {
                loggerWriteLine!("Port multiplier");
            }
            _ => {
                loggerWriteLine!("Dunno what 0x{:X} is", sig);
            }
        }

        return Some(false);
    }

    fn parseIPM(ipm: u32) -> IPM {
        match ipm {
            0 => IPM::NotPresentOrEstablished,
            1 => IPM::Active,
            2 => IPM::Partial,
            6 => IPM::Slumber,
            8 => IPM::DevSleep,
            _ => IPM::Reserved,
        }
    }

    fn praseDET(det: u32) -> DET {
        match det {
            0 => DET::NotDetected,
            1 => DET::DetectedButNotEstablished,
            3 => DET::DetectedAndEstablished,
            4 => DET::Offline,
            _ => DET::Reserved,
        }
    }
}

#[derive(Debug, PartialEq)]
// Interface Power Management
enum IPM {
    NotPresentOrEstablished, // Device not present or communication not established
    Active,                  // Interface in active state
    Partial,                 // Interface in Partial power management state
    Slumber,                 // Interface in Slumber power management state
    DevSleep,                // Interface in DevSleep power management state
    Reserved,
}

#[derive(Debug, PartialEq)]
// Device Detection
enum DET {
    NotDetected,               // No device detected and Phy communication not established
    DetectedButNotEstablished, // Device presence detected but Phy communication not established
    DetectedAndEstablished,    // Device presence detected and Phy communication established
    Offline, // Phy in offline mode as a result of the interface being disabled or running in a BIST loopback mode
    Reserved,
}
