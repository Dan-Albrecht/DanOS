#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![feature(asm_const)]

mod a20Stuff;
mod assemblyStuff;
mod cursorStuff;
mod gdtStuff;
mod pagingStuff;

use core::{panic::PanicInfo, arch::asm};

use a20Stuff::IsTheA20LineEnabled;
use assemblyStuff::cpuID::Is64BitModeSupported;
use cursorStuff::writeStringOnNewline;
use gdtStuff::Setup64BitGDT;
use pagingStuff::enablePaging;

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
                enablePaging();
                writeStringOnNewline(b"64-bit paging mode enabled...");
                writeStringOnNewline(b"...though we're in compatability (32-bit) mode currently.");
                Setup64BitGDT();
                writeStringOnNewline(b"The new GDT is in place");
                asm!(
                    "jmp 0x8, 0xC800" // BUGUBG: Fix this offset hardcoding
                );
            } else {
                writeStringOnNewline(b"No 64-bit mode. :(");
            }
        } else {
            writeStringOnNewline(b"You have hardware/emulator with the A20 address line disabled...");
        }
    };

    loop {}
}
