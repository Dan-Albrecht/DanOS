//use core::fmt::Write;

use kernel_shared::{acpi::rsdp::getRsdp, assemblyStuff::halt::haltLoop};

pub fn readBytes() {
    let _foo = getRsdp();
    haltLoop();
}

