#![no_std]
#![no_main]
#![allow(non_snake_case)]

use core::arch::asm;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        asm!(
            // Some obvious instructions we should be able to easily see in the debugger
            "xor rax, rax",
            "xor rbx, rbx",
            "xor rcx, rcx",
            "xor rdx, rdx",
            "xor r8, r8",
            "xor r9, r9",
            "hlt",
        );
    }

    loop {}
}
