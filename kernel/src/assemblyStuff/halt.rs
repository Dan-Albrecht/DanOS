use core::arch::asm;

pub fn haltLoop() -> ! {
    unsafe {
        loop {
            asm!("hlt");
        }
    }
}
