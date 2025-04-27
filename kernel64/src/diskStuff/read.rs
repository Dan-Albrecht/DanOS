use kernel_shared::assemblyStuff::halt::haltLoop;

use crate::{acpi::rsdp::getRsdp, ahci::controller::Controller, loggerWriteLine, memory::virtualMemory::VirtualMemoryManager};

pub fn readBytes(vmm: &mut VirtualMemoryManager) {
    let rdsp = getRsdp(vmm);
    if let Some(pciGeneralDevice) = rdsp {
        loggerWriteLine!("Maybe can read some bytes from 0x{:X}", pciGeneralDevice as usize);
        if let Some(drive) = Controller::tryGet(pciGeneralDevice) {
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
