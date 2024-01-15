#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![feature(asm_const)]

mod cursorStuff;
mod assemblyStuff;

use core::panic::PanicInfo;

use cursorStuff::writeStringOnNewline;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe { writeStringOnNewline(b"We've made it to Rust!") };

    loop {}
}
