use core::arch::asm;
use core::fmt::Write;
use crate::vgaWrite;

pub fn haltLoop() -> ! {
    vgaWrite!("End of line");
    unsafe {
        loop {
            asm!("hlt");
        }
    }
}

#[macro_export]
macro_rules! haltLoopWithMessage {
    ($($args:tt)*) => {
        $crate::vgaWriteLine!($($args)*);
        haltLoop();
    };
}