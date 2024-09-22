#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![feature(naked_functions)]
#![feature(abi_x86_interrupt)]
#![feature(used_with_arg)]
#![feature(concat_idents)]
#![feature(const_trait_impl)]
#![feature(if_let_guard)]

mod acpi;
mod ahci;
mod assemblyHelpers;
mod diskStuff;
mod interupts;
mod logging;
mod memory;
mod serial;

use core::array::from_fn;
use core::panic::PanicInfo;
use core::{arch::asm, fmt::Write};

use diskStuff::read::readBytes;
use interupts::InteruptDescriptorTable::SetIDT;

use kernel_shared::gdtStuff::GetGdtr;
use kernel_shared::haltLoopWithMessage;
use kernel_shared::magicConstants::{
    PAGES_PER_TABLE, SATA_DRIVE_BASE_CMD_BASE_ADDRESS, SATA_DRIVE_BASE_COMMAND_TABLE_BASE_ADDRESS, SATA_DRIVE_BASE_FIS_BASE_ADDRESS, VGA_BUFFER_ADDRESS, VGA_BYTES_PER_CHAR, VGA_HEIGHT, VGA_WIDTH
};
use kernel_shared::memoryMap::MemoryMap;
use kernel_shared::pageTable::enums::*;
use kernel_shared::physicalMemory::{MemoryBlob, PhysicalMemoryManager, WhatDo};
use kernel_shared::{
    assemblyStuff::{
        halt::haltLoop,
        misc::{Breakpoint, DivideByZero},
    },
    pageTable::pageBook::PageBook,
};
use memory::dumbHeap::BootstrapDumbHeap;
use memory::virtualMemory::VirtualMemoryManager;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // We get called mid-line, so always move to a new one
    loggerWriteLine!("");
    loggerWriteLine!("64-bit kernel panic!");
    loggerWriteLine!("{info}");
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

fn getSP() -> usize {
    unsafe {
        let sp;
        asm!(
            "mov {0}, rsp",
            out(reg) sp,
        );

        sp
    }
}

#[no_mangle]
pub extern "sysv64" fn DanMain(memoryMapLocation: usize) -> ! {
    loggerWriteLine!("Welcome to 64-bit Rust!");

    let memoryMap = MemoryMap::Load(memoryMapLocation.try_into().unwrap());

    let mut physicalMemoryManager = PhysicalMemoryManager {
        MemoryMap: memoryMap,
        Blobs: from_fn(|_| MemoryBlob::default()),
    };

    // BUGBUG: Should probalby get the base pointer as this function has already subtracted stack space
    let sp = getSP();
    physicalMemoryManager.Reserve(0, sp, WhatDo::YoLo);

    // NB: The current secret handshake with the 32-bit code is take the first
    // entry from the memory map. The address of the GDT to the end of that entry
    // has already been used.
    let gdtBase = GetGdtr().BaseAddress;
    physicalMemoryManager.ReserveKernel32(gdtBase);

    // This is probably not in the memory map, but if it shows up, we want to mark it as used
    physicalMemoryManager.Reserve(VGA_BUFFER_ADDRESS.try_into().unwrap(), (VGA_WIDTH * VGA_HEIGHT * VGA_BYTES_PER_CHAR).into(), WhatDo::YoLo);

    const DUMB_HEAP_SIZE: usize = 0x5_0000;
    let dumbHeapAddress: *mut u8 = physicalMemoryManager.ReserveWherever(DUMB_HEAP_SIZE, 1);

    let pageBook = PageBook::fromExisting();
    let bdh = BootstrapDumbHeap::new(dumbHeapAddress as usize, DUMB_HEAP_SIZE);
    loggerWriteLine!("PageBook @ 0x{:X}", pageBook.getCR3Value() as usize);

    loggerWriteLine!("Installing interrupt table...");
    unsafe {
        SetIDT(&mut physicalMemoryManager);
    }
    loggerWriteLine!("Sending a breakpoint...");
    Breakpoint();
    loggerWriteLine!("We handled the breakpoint!");

    /*loggerWriteLine!("Seting up heap...");
    let mut heap = DumbHeap::new(memoryMap);
    let count = 100;
    let myAlloc = heap.DoSomething(count);
    loggerWriteLine!("Allocated 0x{:X} at 0x{:X}", count, myAlloc);

    heap.DumpHeap();*/

    // BUGBUG: Having trouble transfering this to the virtual memory manager
    //let pp = &mut physicalMemoryManager as *mut PhysicalMemoryManager;

    let mut virtualMemoryManager = VirtualMemoryManager::new(physicalMemoryManager, pageBook, bdh);

    // BUGBUG: We're cheating that we know where the disk will be so just page it in
    virtualMemoryManager.identityMap(0x7E0_0000, PAGES_PER_TABLE, WhatDo::Normal);
    virtualMemoryManager.identityMap(0xB000_0000, 0x100, WhatDo::UseReserved);
    virtualMemoryManager.identityMap(0xFEBD_5000, 1, WhatDo::YoLo);
    virtualMemoryManager.identityMap(
        SATA_DRIVE_BASE_CMD_BASE_ADDRESS as usize,
        0x10,
        WhatDo::Normal,
    );
    virtualMemoryManager.identityMap(
        SATA_DRIVE_BASE_FIS_BASE_ADDRESS as usize,
        0x10,
        WhatDo::Normal,
    );
    virtualMemoryManager.identityMap(
        SATA_DRIVE_BASE_COMMAND_TABLE_BASE_ADDRESS,
        0x10,
        WhatDo::Normal,
    );
    virtualMemoryManager.identityMap(0x800_0000, PAGES_PER_TABLE, WhatDo::YoLo);

    reloadCR3();
    readBytes();

    loggerWriteLine!("Now let's divide by 0...");
    DivideByZero();

    loggerWriteLine!("!! We succesfuly divide by zero. We broke.");
    haltLoop();
}
