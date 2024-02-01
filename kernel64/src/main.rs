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
use vga::textMode::writeStringOnNewline;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn DanMain() -> ! {
    writeStringOnNewline(b"Welcome to 64-bit Rust!");
    SetIDT();
    writeStringOnNewline(b"Sending a breakpoint...");
    Breakpoint();
    writeStringOnNewline(b"We handled the breakpoint!");
    DisableInterrupts();

    loop {}
}
