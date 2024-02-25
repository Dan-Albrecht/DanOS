#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![feature(asm_const)]

mod a20Stuff;
mod assemblyStuff;
mod diskStuff;
mod gdtStuff;
mod pagingStuff;

use core::arch::asm;
use core::panic::PanicInfo;

use a20Stuff::IsTheA20LineEnabled;
use assemblyStuff::cpuID::Is64BitModeSupported;
use diskStuff::read::readBytes;
use gdtStuff::Setup64BitGDT;
use kernel_shared::vgaWriteLine;
use pagingStuff::enablePaging;
use core::fmt::Write;

const fn getKernel64Address() -> u16 {
    let bytes = core::env!("KERNEL64_LOAD_TARGET").as_bytes();
    let len = bytes.len();

    if len < 3 || bytes[0] != b'0' || bytes[1] != b'x' {
        assert!(
            false,
            "Load address string must be at least 3 characters and start with a 0x prefix"
        );
    }

    let mut pos = 2;
    let mut val: u16 = 0;

    while pos < len {
        let byte = bytes[pos];
        val <<= 4;

        if byte >= b'0' && byte <= b'9' {
            val += (byte as u16) - (b'0' as u16);
        } else if byte >= b'A' && byte <= b'F' {
            val += 10 + (byte as u16) - (b'A' as u16);
        } else {
            assert!(false, "Invalid character in address string. Hex characters must be in uppercase if you're using them.");
        }
        pos += 1;
    }

    val
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn DanMain() -> ! {
    unsafe {
        // Previous stage didn't newline after its last message
        vgaWriteLine!("\r\nWe've made it to Rust!");
        readBytes();

        if IsTheA20LineEnabled() {
            if Is64BitModeSupported() {
                vgaWriteLine!("64-bit mode is available");
                enablePaging();
                vgaWriteLine!("64-bit paging mode enabled...");
                vgaWriteLine!("...though we're in compatability (32-bit) mode currently.");
                Setup64BitGDT();
                vgaWriteLine!("The new GDT is in place");
                asm!(
                    "jmp 0x8, {adr}", // Far jump to the 64bit kernel
                    adr = const { getKernel64Address() },
                );
            } else {
                vgaWriteLine!("No 64-bit mode. :(");
            }
        } else {
            vgaWriteLine!("You have hardware/emulator with the A20 address line disabled...");
        }
    };

    loop {}
}
