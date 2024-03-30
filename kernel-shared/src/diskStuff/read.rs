use core::fmt::Write;

use crate::{acpi::rsdp::getRsdp, ahci::controller::Controller, assemblyStuff::halt::haltLoop, vgaWriteLine};

pub fn readBytes() {
    let foo = getRsdp();
    if let Some(firstAhci) = foo {
        vgaWriteLine!("Maybe can read some bytes from 0x{:X}", firstAhci as usize);
        if let Some(drive) = Controller::tryGet(firstAhci) {
            drive.stopCommands();
            drive.remapStuff();
            drive.startCommands();
            vgaWriteLine!("Drive is partially configured");
        } else {
            vgaWriteLine!("No drive found");    
        }
    } else {
        vgaWriteLine!("Didn't find an AHCI controller");
    }
    haltLoop();
}
