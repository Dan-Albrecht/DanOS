#![no_std]
#![no_main]

use core::{panic::PanicInfo, arch::asm};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        // Do some random stuff we should easily be able to identify in the debugger to 
        // know we've loaded and executed correctly
        asm!("xor eax, eax");
        asm!("xor ebx, ebx");
        asm!("xor ecx, ecx");
        asm!("xor edx, edx");
        asm!("xor ecx, ecx");
        asm!("xor ebx, ebx");
        asm!("xor eax, eax");
    }
    loop {}
}
