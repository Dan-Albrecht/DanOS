use core::arch::asm;
use core::fmt::Write;
use crate::vgaWrite;

pub fn haltLoop() -> ! {
    vgaWrite!("Halted");
    unsafe {
        loop {
            asm!("hlt");
        }
    }
}
