#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![feature(asm_const)]

mod cursorStuff;
mod assemblyStuff;
mod a20Stuff;

use core::panic::PanicInfo;

use a20Stuff::IsTheA20LineEnabled;
use assemblyStuff::cpuID::Is64BitModeSupported;
use cursorStuff::writeStringOnNewline;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe { 
        writeStringOnNewline(b"We've made it to Rust!");

        if IsTheA20LineEnabled() {
            if Is64BitModeSupported() {
                writeStringOnNewline(b"64-bit mode is available");
            } else {
                writeStringOnNewline(b"No 64-bit mode. :(");
            }
        } else {
            writeStringOnNewline(b"You have hardware/emulator with the A20 address line disabled...");
        }
    };

    loop {}
}
