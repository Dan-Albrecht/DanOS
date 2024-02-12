use crate::vgaWriteLine;
use core::{fmt::Write, mem::size_of};

#[repr(C, packed)]
pub struct MemoryMap {}

#[repr(C, packed)]
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

        vgaWriteLine!(
            "We should read 0x{:X} entries from 0x{:X} + 16 byte",
            totalEntries,
            address
        );

        let mut entryAddress = address + 0x10;

        for index in 0..totalEntries {
            let ptr = entryAddress as *const MemoryMapEntry;
            let entry: MemoryMapEntry;

            unsafe {
                entry = ptr.read_unaligned();
            }

            let addr = entry.BaseAddr;
            let length = entry.Length;
            let typee = entry.Type;
            let at = entry.Attributes;
            vgaWriteLine!(
                "{}: Base - 0x{:X} Length - 0x{:X} Type:0x{:X} At:0x{:X}",
                index,
                addr,
                length,
                typee,
                at
            );

            entryAddress += size_of::<MemoryMapEntry>();
        }

        let result = MemoryMap{};
        return  result;
    }
}
