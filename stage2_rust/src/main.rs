#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![feature(log_syntax)]
#![feature(cfg_relocation_model)]

mod disk;
mod memory;

use core::{arch::asm, panic::PanicInfo};
use disk::{diskDriver::DiskDriver, fatDriver::FatDriver};
use kernel_shared::{
    assemblyStuff::{halt::haltLoop, misc::disablePic}, gdtStuff::Gdt, haltLoopWithMessage, textMode::teletype, vgaWrite, vgaWriteLine
};
use memory::map::MemoryMap;

#[panic_handler]
fn panic(pi: &PanicInfo) -> ! {
    teletype::printLine(b"16-bit panic!");

    if let Some(msg) = pi.message().as_str() {
        teletype::printLine(msg.as_bytes());
    } else {
        teletype::printLine(b"Couldn't get panic message easily; trying harder");
        // We're risking a further panic here, but really want to see the message
        haltLoopWithMessage!("Panic: {:?}", pi);
    }

    teletype::printLine(b"End of line");

    unsafe {
        loop {
            asm!("hlt");
        }
    }
}

#[cfg(debug_assertions)]
fn sayHello() {
    teletype::printLine(b"Hi from 16-bit Debug Rust!");
}

#[cfg(not(debug_assertions))]
fn sayHello() {
    teletype::printLine(b"Hi from 16-bit Release Rust!");
}

#[unsafe(no_mangle)]
pub extern "fastcall" fn DanMain(driveNumber: u32) -> ! {
    #[cfg(not(relocation_model = "static"))]
    compile_error!("Stage1 boot loader cannot handle having to relocate us.");

    disablePic();
    sayHello();

    // We need full 32-bit segment offsets to access everything as this code
    // doesn't compile in a way that it knows to manipulate the segment registers.
    // Only static strings should be used before this switch as fmt loves to
    // try and jump somwhere we cannot yet reach.
    let gdt = Gdt::create32BitFlat();
    unsafe { gdt.enterUnrealMode(); };

    vgaWriteLine!("Running in Unreal mode");

    let mm: MemoryMap;

    unsafe {
        match MemoryMap::create() {
            Ok(result) => mm = result,
            Err(msg) => haltLoopWithMessage!("Getting memory map failed: {}", msg),
        }
    }

    mm.dump();

    let disk = DiskDriver::new(driveNumber.try_into().unwrap());

    let fat = FatDriver::new(disk);
    let kernel32 = fat.findKernel32().unwrap();
    vgaWriteLine!("Kernel32 is at 0x{:X}", kernel32);
    haltLoopWithMessage!("End of current 16-bit code");
    
}
