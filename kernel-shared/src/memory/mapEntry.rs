use crate::{loggerWriteLine, vgaWriteLine};

#[derive(Debug, PartialEq)]
pub enum MemoryMapEntryType {
    AddressRangeMemory,
    AddressRangeReserved,
    AddressRangeACPI,
    AddressRangeNVS,
    AddressRangeUnusable,
    AddressRangeDisabled,
    AddressRangePersistentMemory,
    Undefined,
    OemDefined,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct MemoryMapEntry {
    pub BaseAddress: u64,
    pub Length: u64,
    pub Type: u32,
    pub ExtendedAttributes: u32,
}

// https://uefi.org/htmlspecs/ACPI_Spec_6_4_html/15_System_Address_Map_Interfaces/Sys_Address_Map_Interfaces.html#address-range-types
impl MemoryMapEntry {
    pub fn getType(&self) -> MemoryMapEntryType {
        match self.Type {
            0 => MemoryMapEntryType::Undefined,
            1 => MemoryMapEntryType::AddressRangeMemory,
            2 => MemoryMapEntryType::AddressRangeReserved,
            3 => MemoryMapEntryType::AddressRangeACPI,
            4 => MemoryMapEntryType::AddressRangeNVS,
            5 => MemoryMapEntryType::AddressRangeUnusable,
            6 => MemoryMapEntryType::AddressRangeDisabled,
            7 => MemoryMapEntryType::AddressRangePersistentMemory,
            8..=11 => MemoryMapEntryType::Undefined,
            12 => MemoryMapEntryType::OemDefined,
            13..=0xEFFFFFFF => MemoryMapEntryType::Undefined,
            0xF0000000..=0xFFFFFFFF => MemoryMapEntryType::OemDefined,
        }
    }

    pub fn dump(&self) {
        self.dumpEx(false);
    }

    pub fn dumpEx(&self, useLogger: bool) {
        let baseAddress = self.BaseAddress;

        let mut endAddress = self.BaseAddress + self.Length;
        if endAddress != 0 {
            // The end address is inclusive, so we need to subtract one to get the last address.
            // However if it is 0, don't do that as it'd underflow.
            endAddress -= 1;
        }

        let length = self.Length;

        if useLogger {
            loggerWriteLine!(
                "{:?} 0x{:X} - 0x{:X} (0x{:X})",
                self.getType(),
                baseAddress,
                endAddress,
                length,
            );
        } else {
            vgaWriteLine!(
                "{:?} 0x{:X} - 0x{:X} (0x{:X})",
                self.getType(),
                baseAddress,
                endAddress,
                length,
            );
        }
    }
}
