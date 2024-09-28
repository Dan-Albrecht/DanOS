use core::{fmt::Write, mem::size_of};

use crate::{assemblyStuff::halt::haltLoop, haltLoopWithMessage, vgaWriteLine};

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

// MemoryMap tells us about the physical memory avilable on the machine
#[derive(Copy, Clone)]
pub struct MemoryMap {
    // BUGBUG: Figure out how to cojule this without hardcoding a waste
    pub Entries: [MemoryMapEntry; 32],
    pub Count: u8,
}

#[repr(C, packed)]
#[derive(Default, Copy, Clone)]
pub struct MemoryMapEntry {
    // https://uefi.org/htmlspecs/ACPI_Spec_6_4_html/15_System_Address_Map_Interfaces/int-15h-e820h---query-system-address-map.html
    pub BaseAddr: u64,
    pub Length: u64,
    pub Type: u32,
    pub Attributes: u32,
}

impl MemoryMap {
    pub fn Load(address: usize) -> MemoryMap {
        let totalEntries: usize;
        unsafe {
            totalEntries = *(address as *const usize);
        }

        let mut entryAddress = address + 0x10;

        vgaWriteLine!(
            "We should read 0x{:X} entries from 0x{:X}",
            totalEntries,
            entryAddress
        );

        let mut result = MemoryMap {
            Count: totalEntries as u8,
            Entries: Default::default(),
        };

        if totalEntries >= result.Entries.len() {
            haltLoopWithMessage!("Num of ({}) entries is bogus of we need to update code to handle", totalEntries);
        }

        for index in 0..totalEntries {
            let ptr = entryAddress as *const MemoryMapEntry;
            let entry: MemoryMapEntry;

            unsafe {
                entry = ptr.read_unaligned();
            }

            let at = entry.Attributes;
            result.Entries[index].Attributes = at;

            let addr = entry.BaseAddr;
            result.Entries[index].BaseAddr = addr;

            let length = entry.Length;
            result.Entries[index].Length = length;

            let entryType = entry.Type;
            result.Entries[index].Type = entryType;

            entryAddress += size_of::<MemoryMapEntry>();
        }

        return result;
    }

    pub fn Dump(&self) {
        // We do the loop backwards, because if there's a crash we'll hope the first few entries are on the screen (especially if we don't have serial hooked up)
        let mut index = self.Count as usize - 1;

        loop {
            let addr = self.Entries[index].BaseAddr;
            let len = self.Entries[index].Length;

            vgaWriteLine!(
                "{}: {:?} 0x{:X} - 0x{:X} (0x{:X})",
                index,
                self.Entries[index].GetType(),
                addr,
                addr + len - 1,
                len,
            );

            if index == 0 {
                break;
            }

            index -= 1;
        }

        vgaWriteLine!("size is {}", self.Count);
    }

    pub fn IsValid(
        &self,
        requestedAddress: u64,
        requestedLength: u64,
        requestedType: MemoryMapEntryType,
    ) -> bool {
        for i in 0..self.Count as usize {
            let base = self.Entries[i].BaseAddr;
            let length = self.Entries[i].Length;
            let memType = self.Entries[i].GetType();

            if (base <= requestedAddress)
                && ((requestedAddress + requestedLength) <= (base + length))
            {
                if memType == requestedType {
                    return true;
                } else {
                    return false;
                }
            }
        }

        return false;
    }
}

// https://uefi.org/htmlspecs/ACPI_Spec_6_4_html/15_System_Address_Map_Interfaces/Sys_Address_Map_Interfaces.html#address-range-types
impl MemoryMapEntry {
    pub fn GetType(&self) -> MemoryMapEntryType {
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
}
