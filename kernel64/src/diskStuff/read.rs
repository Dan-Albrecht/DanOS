use core::fmt::Write;

use kernel_shared::assemblyStuff::halt::haltLoop;

use crate::{acpi::rsdp::getRsdp, ahci::controller::Controller, loggerWriteLine, vgaWriteLine};

pub fn readBytes() {
    let foo = getRsdp();
    if let Some(firstAhci) = foo {
        loggerWriteLine!("Maybe can read some bytes from 0x{:X}", firstAhci as usize);
        if let Some(drive) = Controller::tryGet(firstAhci) {
            drive.stopCommands();
            drive.remapStuff();
            drive.startCommands();
            loggerWriteLine!("Drive is partially configured");
        } else {
            loggerWriteLine!("No drive found");    
        }
    } else {
        loggerWriteLine!("Didn't find an AHCI controller");
    }
    haltLoop();
}
