use core::str::from_utf8;

use kernel_shared::{assemblyStuff::halt::haltLoop, memoryHelpers::alignDown, pageTable::enums::*};

use crate::{loggerWriteLine, memory::virtualMemory::VirtualMemoryManager};

use super::{pciGeneralDevice::PciGeneralDevice, rsdt::RSDT};

// https://uefi.org/specs/ACPI/6.5/05_ACPI_Software_Programming_Model.html#root-system-description-pointer-rsdp-structure
// Version 1 (Revsion 0) defintion
// Root System Description Pointer
#[repr(C, packed)]
pub struct RSDP {
    Signature: [u8; 8],
    Checksum: u8,
    OEMID: [u8; 6],
    Revision: u8,
    RsdtAddress: u32,
}

pub fn getRsdp(vmm: &mut VirtualMemoryManager) -> Option<*const PciGeneralDevice> {
    // https://uefi.org/specs/ACPI/6.5/05_ACPI_Software_Programming_Model.html#finding-the-rsdp-on-ia-pc-systems
    // Going to assume this isn't in the Extended BIOS Data Area (EBDA) and search directly in the BIOS read-only memory

    let physicalAddress: usize = 0xE_0000;
    let length: usize = 0x2_0000;
    let mut virtualAddress: usize = 0xD00_0000;
    vmm.map(
        physicalAddress,
        virtualAddress,
        length,
        Execute::Yes,
        Present::Yes,
        Writable::Yes,
        Cachable::No,
        UserSupervisor::Supervisor,
        WriteThrough::WriteTrough,
    );

    loggerWriteLine!("Mapped BIOS");

    loop {
        let ptr = virtualAddress as *const RSDP;
        let checkMe = checkSignature(ptr, vmm);
        if let Ok(xxx) = checkMe {
            return xxx;
        }

        virtualAddress = virtualAddress + 16;
        if virtualAddress >= (virtualAddress + length) {
            loggerWriteLine!("Didn't find RSDP. Halting.");
            haltLoop();
        }
    }
}

fn checkSignature(ptr: *const RSDP, vmm: &mut VirtualMemoryManager) -> Result<Option<*const PciGeneralDevice>, u8> {
    let expected = *b"RSD PTR ";
    unsafe {
        let toCheck = (*ptr).Signature;

        if toCheck == expected {
            loggerWriteLine!("Potential ACPI info at: 0x{:X}", ptr as usize);

            let mut calculated: u8 = 0;
            let asBytes = ptr as *const u8;
            for index in 0..20 {
                let byte = *asBytes.offset(index);
                calculated = calculated.wrapping_add(byte);
            }

            if calculated != 0 {
                loggerWriteLine!("Checksum fail (should be 0): {calculated}");
                return Err(1);
            }

            match from_utf8(&(*ptr).OEMID) {
                Ok(theString) => {
                    loggerWriteLine!("ACPI by {}", theString);

                    // Spec says this is always a 32 bit address
                    loggerWriteLine!("RSDT is at 0x{:X}", (*ptr).RsdtAddress as u32);
                }
                _ => {
                    loggerWriteLine!("Couldn't read ACPI OEM: {:?}", (*ptr).OEMID);
                }
            };

            // BUGBUG: Delete after debugging
            let rsdt :usize= (*ptr).RsdtAddress.try_into().unwrap();
            let aligned = alignDown(rsdt.try_into().unwrap(), 0x1000);
            let diff = rsdt - aligned;

            let length: usize = 0x1_0000;
            let virtualAddress = vmm.mapPhysicalAnywhere(
                aligned,
                length,
                Execute::Yes,
                Present::Yes,
                Writable::Yes,
                Cachable::No,
                UserSupervisor::Supervisor,
                WriteThrough::WriteTrough,
            );

            let rsdt = (virtualAddress + diff) as *const RSDT;



            let result = (*rsdt).walkEntries();
            return Ok(result);
        }
    }

    return Err(2);
}
