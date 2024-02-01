use core::arch::asm;

pub fn Breakpoint() {
    unsafe {
        asm!("int 3");
    }
}
