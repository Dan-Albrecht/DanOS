use core::mem::size_of;

use crate::{assemblyStuff::halt::haltLoop, loggerWriteLine, vgaWriteLine};

pub unsafe fn zeroMemory(address: usize, ammount: usize) {
    unsafe {
        loggerWriteLine!("zeroMemory: 0x{:X} ammount: 0x{:X}", address, ammount);
        core::ptr::write_bytes(address as *mut u8, 0, ammount);
    }
}

pub unsafe fn zeroMemory2<T>(address: *const T) {
    unsafe {
        let address = address as usize;
        let ammount = size_of::<T>();
        zeroMemory(address, ammount);
    }
}

pub fn haltOnMisaligned(msg: &'static str, address: usize, alignment: usize) {
    let mask = alignment - 1;
    if address & mask != 0 {
        vgaWriteLine!("{} 0x{:X} is not 0x{:X} aligned", msg, address, alignment);
        let theMod = address % alignment;
        let low = address - theMod;
        let high = low + alignment;
        vgaWriteLine!("Maybe 0x{:X} or 0x{:X}", low, high);
        haltLoop();
    }
}

pub fn alignUp(address: usize, alignment: usize) -> usize {
    let theMod = address % alignment;
    if theMod == 0 {
        return address;
    }
    let low = address - theMod;
    let high = low + alignment;
    return high;
}

pub fn alignDown(address: usize, alignment: usize) -> usize {
    let theMod = address % alignment;
    if theMod == 0 {
        return address;
    }
    let low = address - theMod;

    return low;
}
