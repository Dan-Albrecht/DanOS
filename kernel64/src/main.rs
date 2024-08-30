#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![feature(naked_functions)]
#![feature(abi_x86_interrupt)]
#![feature(used_with_arg)]
#![feature(concat_idents)]
#![feature(const_trait_impl)]

mod assemblyHelpers;
mod interupts;
mod memory;
mod pic;

use core::panic::PanicInfo;
use core::{arch::asm, fmt::Write};

use interupts::InteruptDescriptorTable::SetIDT;
use kernel_shared::{
    assemblyStuff::{
        halt::haltLoop,
        misc::{Breakpoint, DivideByZero},
    },
    diskStuff::read::readBytes,
    magicConstants::MEMORY_MAP_LOCATION,
    pageTable::pageBook::PageBook,
    vgaWriteLine,
};
use memory::memoryMap::MemoryMap;
use memory::physicalMemory::PhysicalMemoryManager;
use memory::virtualMemory::VirtualMemoryManager;

use crate::{memory::dumbHeap::DumbHeap, pic::picStuff::disablePic};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vgaWriteLine!("64-bit kernel panic!");
    vgaWriteLine!("{info}");
    haltLoop();
}

fn reloadCR3() {
    unsafe {
        asm!(
            "mov rax, cr3",
            "mov cr3, rax",
            out("rax") _,
        );
    }
}

#[no_mangle]
pub extern "C" fn DanMain() -> ! {
    vgaWriteLine!("Welcome to 64-bit Rust!");

    let memoryMap = MemoryMap::Load(MEMORY_MAP_LOCATION);
    let physicalMemoryManager = PhysicalMemoryManager::Init(memoryMap);

    let pageBook: PageBook;
    unsafe {
        pageBook = PageBook::fromExisting64();
    }

    vgaWriteLine!("PageBook @ 0x{:X}", pageBook.getCR3Value() as usize);
    let virtualMemoryManager = VirtualMemoryManager::new(physicalMemoryManager, pageBook);

    vgaWriteLine!("Configuring PIC...");
    disablePic();

    vgaWriteLine!("Installing interrupt table...");
    unsafe {
        SetIDT(physicalMemoryManager);
    }
    vgaWriteLine!("Sending a breakpoint...");
    Breakpoint();
    vgaWriteLine!("We handled the breakpoint!");

    /*vgaWriteLine!("Seting up heap...");
    let mut heap = DumbHeap::new(memoryMap);
    let count = 100;
    let myAlloc = heap.DoSomething(count);
    vgaWriteLine!("Allocated 0x{:X} at 0x{:X}", count, myAlloc);

    heap.DumpHeap();*/

    // BUGBUG: We're cheating that we know where the disk will be so just page it in
    // Need to handle this for real
    // 000 = 0        .. 01F_FFFF
    // 001 = 0020_FFFF .. 03F_FFFF
    // 00F = 01E0_0000 .. 21E_FFFF
    // 020 = 0400_0000 .. 41F_0000
    // 03F = 07E0_0000 .. 7FF_FFFF
    // 1FF = 3FE0_0000 .. 3FFF_FFFF
    // B000_0000
    virtualMemoryManager.identityMap(0x7E0_0000, false);
    virtualMemoryManager.identityMap(0xB000_0000, true);
    //virtualMemoryManager.identityMap(0xFEBD_500C);
    reloadCR3();
    readBytes();

    vgaWriteLine!("Now let's divide by 0...");
    DivideByZero();

    vgaWriteLine!("!! We succesfuly divide by zero. We broke.");
    haltLoop();
}
