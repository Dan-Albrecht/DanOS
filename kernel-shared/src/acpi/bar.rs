use crate::vgaWriteLine;
use core::{fmt::Write, ptr::read_volatile};

pub struct Bar {
    pub BarTarget: u32, // Where the BAR points to with metadata bits masked off
    Value: u32,         // Original value of the BAR value with the metadata bits intact
    AddressSpace: u32,  // The extend of memory this points to
}
impl Bar {
    pub fn new(address: u32, barValue: u32, barAddress: *mut u32) -> Bar {
        unsafe {
            *barAddress = 0xFFFFFFFF;
            let readBack = read_volatile(barAddress);

            // Restore to what it was
            *barAddress = barValue;

            // Since this is a memory address, clear the last 3 bits (the 'info bits')
            let size = (!(readBack & 0xFFFFFFF8)) + 1;

            vgaWriteLine!("Readback was 0x{:X} size is 0x{:X}", readBack, size);

            return Bar {
                BarTarget: address,
                AddressSpace: size,
                Value: barValue,
            };
        }
    }
}
