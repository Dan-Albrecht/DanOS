use core::fmt::Write;
use core::mem::size_of;

use crate::{assemblyStuff::halt::haltLoop, vgaWriteLine};

pub unsafe fn zeroMemory(address: usize, ammount: usize) {
    assert!(ammount <= isize::MAX as usize);
    let pointer = address as *mut u8;
    for index in 0..ammount as isize {
        *pointer.offset(index) = 0;
    }
}

pub unsafe fn zeroMemory2<T>(address: *const T) {
    let address = address as usize;
    let ammount = size_of::<T>();
    zeroMemory(address, ammount);
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
