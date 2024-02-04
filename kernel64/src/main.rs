#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![feature(asm_const)]
#![feature(naked_functions)]
#![feature(abi_x86_interrupt)]
#![feature(used_with_arg)]
#![feature(concat_idents)]
#![feature(const_trait_impl)]

mod assemblyHelpers;
mod interupts;
mod vga;

use core::panic::PanicInfo;

use assemblyHelpers::breakpoint::Breakpoint;
use interupts::InteruptDescriptorTable::{DisableInterrupts, SetIDT};
use core::fmt::Write;

use crate::assemblyHelpers::breakpoint::{DivideByZero, HaltLoop};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn DanMain() -> ! {

    // Previous stage left the cursor on the last line
    vgaWriteLine!("\r\nWelcome to 64-bit Rust!");
    vgaWriteLine!("Installing interrupt table...");
    SetIDT();
    vgaWriteLine!("Sending a breakpoint...");
    Breakpoint();
    vgaWriteLine!("We handled the breakpoint!");
    
    DivideByZero();

    DisableInterrupts();
    HaltLoop();
}
