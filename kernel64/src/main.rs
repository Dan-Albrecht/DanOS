#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![feature(asm_const)]
#![feature(naked_functions)]
#![feature(abi_x86_interrupt)]
#![feature(used_with_arg)]

mod assemblyHelpers;
mod interupts;
mod vga;

use core::panic::PanicInfo;

use assemblyHelpers::breakpoint::Breakpoint;
use interupts::InteruptDescriptorTable::{DisableInterrupts, SetIDT};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn DanMain() -> ! {

    // Previous stage left the cursor on the last line
    vgaWriteLine!("\r\nWelcome to 64-bit Rust!");
    vgaWriteLine!("This is the write line {}!", 2);
    SetIDT();
    vgaWriteLine!("Sending a breakpoint...");
    Breakpoint();
    vgaWriteLine!("We handled the breakpoint!");
    DisableInterrupts();

    loop {}
}
