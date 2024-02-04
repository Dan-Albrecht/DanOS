use core::arch::asm;
use core::fmt::Write;

use crate::vgaWrite;

pub fn Breakpoint() {
    unsafe {
        asm!("int 3");
    }
}

pub fn HaltLoop() -> ! {
    vgaWrite!("Halted");

    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

pub fn DivideByZero() {
    unsafe {
        asm!("xor bx, bx", "div bx",);
    }
}
