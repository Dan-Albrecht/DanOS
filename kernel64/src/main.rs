#![no_std]
#![no_main]
#![allow(non_snake_case)]

mod assemblyHelpers;
mod vga;

use core::panic::PanicInfo;

use vga::textMode::writeStringOnNewline;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn DanMain() -> ! {

    unsafe{
        writeStringOnNewline(b"Welcome to 64-bit Rust!");
    }

    loop {}
}
