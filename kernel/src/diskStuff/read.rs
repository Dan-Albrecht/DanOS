use kernel_shared::{acpi::rsdp::getRsdp, ahci::controller::Controller, assemblyStuff::halt::haltLoop, vgaWriteLine};
use core::fmt::Write;

pub fn readBytes() {
    let foo = getRsdp();
    if let Some(firstAhci) = foo {
        vgaWriteLine!("Maybe can read some bytes from 0x{:X}", firstAhci as usize);
        _ = Controller::tryGet(firstAhci);
    } else {
        vgaWriteLine!("Didn't find an AHCI controller");
    }
    haltLoop();
}
