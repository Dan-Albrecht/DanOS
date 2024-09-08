const PIC1_DATA: u16 = 0x21;
const PIC2_DATA: u16 = 0xA1;

use core::arch::asm;

use super::ports::outB;

pub fn Breakpoint() {
    unsafe {
        asm!("int 3");
    }
}

pub fn DivideByZero() {
    unsafe {
        asm!("xor bx, bx", "div bx",);
    }
}

pub fn disablePic() {
    unsafe {
        outB(PIC1_DATA, 0xFF);
        outB(PIC2_DATA, 0xFF);
    }
}
