#![no_std]
#![no_main]
#![allow(non_snake_case)]

use core::{arch::asm, panic::PanicInfo};

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

fn printChar(char: u8) {
    unsafe {
        asm!(
            "mov ah, 0x0E", // Teletype output function
            "xor bx, bx",   // BH = page number (0), BL is N/A for this mode
                            // so 0 it for consistency
            "int 0x10",     // Video Services
            out("ah") _,
            out("bx") _,
            in("al") char,  // Char to print
        );
    }
}

fn pringString(blah: &[u8]) {
    for b in blah {
        printChar(*b);
    }
}

#[cfg(debug_assertions)]
fn sayHello() {
    pringString(b"Hi from 16-bit Debug Rust!");
}

#[cfg(not(debug_assertions))]
fn sayHello() {
    pringString(b"Hi from 16-bit Release Rust!");
}

#[unsafe(no_mangle)]
pub extern "C" fn DanMain() -> ! {
    sayHello();

    loop {}
}
