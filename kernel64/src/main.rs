#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![feature(naked_functions)]
#![feature(abi_x86_interrupt)]
#![feature(used_with_arg)]
#![feature(concat_idents)]
#![feature(const_trait_impl)]
#![feature(if_let_guard)]

mod assemblyHelpers;
mod interupts;
mod memory;
mod pic;
mod serial;

use core::array::from_fn;
use core::panic::PanicInfo;
use core::{arch::asm, fmt::Write};

use interupts::InteruptDescriptorTable::SetIDT;

use kernel_shared::haltLoopWithMessage;
use kernel_shared::magicConstants::{
    DUMB_HEAP, DUMB_HEAP_LENGTH, PAGES_PER_TABLE, SATA_DRIVE_BASE_CMD_BASE_ADDRESS,
    SATA_DRIVE_BASE_COMMAND_TABLE_BASE_ADDRESS, SATA_DRIVE_BASE_FIS_BASE_ADDRESS,
};
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
use memory::dumbHeap::BootstrapDumbHeap;
use memory::memoryMap::MemoryMap;
use memory::physicalMemory::{MemoryBlob, PhysicalMemoryManager};
use memory::virtualMemory::{VirtualMemoryManager, WhatDo};
use serial::serialPort::{COMPort, SerialPort};

use crate::pic::picStuff::disablePic;

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
    let serial = SerialPort::tryGet(COMPort::COM1);
    if let Some(thePort) = serial {
        thePort.SendLine(b"Hello, Serial Port!!!");
    } else {
        haltLoopWithMessage!("Failed to get the port");
    }

    let memoryMap = MemoryMap::Load(MEMORY_MAP_LOCATION);
    let mut physicalMemoryManager = PhysicalMemoryManager {
        MemoryMap: memoryMap,
        Blobs: from_fn(|_| MemoryBlob::default()),
    };

    let pageBook: PageBook;
    unsafe {
        pageBook = PageBook::fromExisting64();
    }

    let bdh = BootstrapDumbHeap::new(DUMB_HEAP, DUMB_HEAP_LENGTH);
    vgaWriteLine!("PageBook @ 0x{:X}", pageBook.getCR3Value() as usize);

    vgaWriteLine!("Configuring PIC...");
    disablePic();

    vgaWriteLine!("Installing interrupt table...");
    unsafe {
        SetIDT(&mut physicalMemoryManager);
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

    // BUGBUG: Having trouble transfering this to the virtual memory manager
    //let pp = &mut physicalMemoryManager as *mut PhysicalMemoryManager;

    let mut virtualMemoryManager = VirtualMemoryManager::new(physicalMemoryManager, pageBook, bdh);

    // BUGBUG: We're cheating that we know where the disk will be so just page it in
    virtualMemoryManager.identityMap(0x7E0_0000, PAGES_PER_TABLE, WhatDo::YoLo);
    virtualMemoryManager.identityMap(0xB000_0000, 0x100, WhatDo::YoLo);
    virtualMemoryManager.identityMap(0xFEBD_5000, 1, WhatDo::YoLo);
    virtualMemoryManager.identityMap(
        SATA_DRIVE_BASE_CMD_BASE_ADDRESS as usize,
        0x10,
        WhatDo::YoLo,
    );
    virtualMemoryManager.identityMap(
        SATA_DRIVE_BASE_FIS_BASE_ADDRESS as usize,
        0x10,
        WhatDo::YoLo,
    );
    virtualMemoryManager.identityMap(
        SATA_DRIVE_BASE_COMMAND_TABLE_BASE_ADDRESS,
        0x10,
        WhatDo::YoLo,
    );
    /*virtualMemoryManager.identityMap(
        0x7FE_0000,
        0x20,
        WhatDo::YoLo,
    );
    virtualMemoryManager.identityMap(
        0x7FE_2000,
        0x10,
        WhatDo::YoLo,
    );*/
    virtualMemoryManager.identityMap(0x800_0000, PAGES_PER_TABLE, WhatDo::YoLo);
    /*virtualMemoryManager.identityMap(
        0x800_0000,
        PAGES_PER_TABLE,
        WhatDo::UseReserved,
    );*/

    reloadCR3();
    readBytes();

    vgaWriteLine!("Now let's divide by 0...");
    DivideByZero();

    vgaWriteLine!("!! We succesfuly divide by zero. We broke.");
    haltLoop();
}
