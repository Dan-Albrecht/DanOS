#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![feature(naked_functions)]
#![feature(abi_x86_interrupt)]
#![feature(used_with_arg)]
#![feature(concat_idents)]
#![feature(const_trait_impl)]

mod interupts;
mod magicConstants;
mod memory;
mod pic;

use core::fmt::Write;
use core::panic::PanicInfo;

use interupts::InteruptDescriptorTable::SetIDT;
use kernel_shared::{
    assemblyStuff::{
        halt::haltLoop,
        misc::{Breakpoint, DivideByZero},
    }, diskStuff::read::readBytes, pageTable::pageBook::PageBook, vgaWriteLine
};
use memory::memoryMap::MemoryMap;

use magicConstants::MEMORY_MAP_LOCATION;

use crate::{memory::dumbHeap::DumbHeap, pic::picStuff::disablePic};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vgaWriteLine!("64-bit kernel panic!");
    vgaWriteLine!("{info}");
    haltLoop();
}

#[no_mangle]
pub extern "C" fn DanMain() -> ! {
    vgaWriteLine!("Welcome to 64-bit Rust!");

    let memoryMap = MemoryMap::Load(MEMORY_MAP_LOCATION);
    let pageBook: PageBook;
    unsafe {
        pageBook = PageBook::fromExisting64();
    }

    vgaWriteLine!("PageBook @ 0x{:X}", pageBook.getCR3Value() as usize);

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

    readBytes();

    vgaWriteLine!("Now let's divide by 0...");
    DivideByZero();

    vgaWriteLine!("!! We succesfuly divide by zero. We broke.");
    haltLoop();
}
