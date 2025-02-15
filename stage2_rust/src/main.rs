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
    assemblyStuff::{halt::haltLoop, misc::disablePic},
    haltLoopWithMessage,
    textMode::teletype,
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

    let mm: MemoryMap;

    unsafe {
        match MemoryMap::create() {
            Ok(result) => mm = result,
            Err(msg) => haltLoopWithMessage!("Getting memory map failed: {}", msg),
        }
    }

    mm.dump();

    let disk = DiskDriver::new(driveNumber);
    disk.doStuff();

    let fat = FatDriver::new(disk);
    fat.doStuff();

    haltLoopWithMessage!("End of current 16-bit code");
}
