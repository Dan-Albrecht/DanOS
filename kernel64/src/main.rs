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
mod pic;
mod magicConstants;
mod memory;
mod vga;

use core::fmt::Write;
use core::panic::PanicInfo;

use assemblyHelpers::breakpoint::{Breakpoint, DivideByZero, HaltLoop};
use interupts::InteruptDescriptorTable::{DisableInterrupts, SetIDT};
use memory::memoryMap::MemoryMap;

use magicConstants::MEMORY_MAP_LOCATION;

use crate::pic::picStuff::disablePic;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn DanMain() -> ! {
    // Previous stage left the cursor on the last line
    vgaWriteLine!("\r\nWelcome to 64-bit Rust!");

    MemoryMap::Load(MEMORY_MAP_LOCATION);
    //x.Display();

    vgaWriteLine!("Configuring PIC...");
    disablePic();

    vgaWriteLine!("Installing interrupt table...");
    SetIDT();
    vgaWriteLine!("Sending a breakpoint...");
    Breakpoint();
    vgaWriteLine!("We handled the breakpoint! Now let's divide by 0...");
    DivideByZero();

    vgaWriteLine!("!! We succesfuly divide by zere. We broke.");
    DisableInterrupts();
    HaltLoop();
}
