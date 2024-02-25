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

use core::fmt::Write;
use core::panic::PanicInfo;

use assemblyHelpers::breakpoint::{Breakpoint, DivideByZero, HaltLoop};
use interupts::InteruptDescriptorTable::{DisableInterrupts, SetIDT};
use kernel_shared::vgaWriteLine;
use memory::memoryMap::MemoryMap;

use magicConstants::MEMORY_MAP_LOCATION;

use crate::{memory::dumbHeap::DumbHeap, pic::picStuff::disablePic};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn DanMain() -> ! {
    // Previous stage left the cursor on the last line
    vgaWriteLine!("\r\nWelcome to 64-bit Rust!");

    let memoryMap = MemoryMap::Load(MEMORY_MAP_LOCATION);
    //x.Display();

    vgaWriteLine!("Configuring PIC...");
    disablePic();

    vgaWriteLine!("Installing interrupt table...");
    SetIDT();
    vgaWriteLine!("Sending a breakpoint...");
    Breakpoint();
    vgaWriteLine!("We handled the breakpoint!");

    vgaWriteLine!("Seting up heap...");
    let mut heap = DumbHeap::new(memoryMap);
    let count = 100;
    let myAlloc = heap.DoSomething(count);
    vgaWriteLine!("Allocated 0x{:X} at 0x{:X}", count, myAlloc);

    heap.DumpHeap();

    vgaWriteLine!("Now let's divide by 0...");
    DivideByZero();

    vgaWriteLine!("!! We succesfuly divide by zere. We broke.");
    DisableInterrupts();
    HaltLoop();
}
