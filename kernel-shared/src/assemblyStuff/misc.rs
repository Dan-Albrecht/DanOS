use core::arch::asm;

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
