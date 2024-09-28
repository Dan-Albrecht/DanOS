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
mod magicConstants;
mod memory;
mod serial;

use core::array::from_fn;
use core::mem;
use core::panic::PanicInfo;
use core::ptr::{read_unaligned, write_unaligned};
use core::{arch::asm, fmt::Write};

use diskStuff::read::readBytes;
use interupts::InteruptDescriptorTable::SetIDT;

use kernel_shared::gdtStuff::GetGdtr;
use kernel_shared::magicConstants::*;
use kernel_shared::memoryMap::MemoryMap;
use kernel_shared::pageTable::enums::*;
use kernel_shared::physicalMemory::{MemoryBlob, PhysicalMemoryManager, WhatDo};
use kernel_shared::relocation::relocateKernel64;
use kernel_shared::{
    assemblyStuff::{
        halt::haltLoop,
        misc::{Breakpoint, DivideByZero},
    },
    pageTable::pageBook::PageBook,
};
use kernel_shared::{haltLoopWithMessage, vgaWriteLine};
use magicConstants::*;
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
pub extern "sysv64" fn DanMain(memoryMapLocation: usize, kernelSize: usize) -> ! {
    loggerWriteLine!(
        "Welcome to 64-bit Rust! We're 0x{:X} bytes long.",
        kernelSize
    );

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
    physicalMemoryManager.Reserve(
        VGA_BUFFER_ADDRESS.try_into().unwrap(),
        (VGA_WIDTH * VGA_HEIGHT * VGA_BYTES_PER_CHAR).into(),
        WhatDo::YoLo,
    );

    const DUMB_HEAP_SIZE: usize = 0x5_0000;
    let dumbHeapAddress: *mut u8 = physicalMemoryManager.ReserveWherever(DUMB_HEAP_SIZE, 1);
    loggerWriteLine!(
        "Dumb heap @ 0x{:X} for 0x{:X}",
        dumbHeapAddress as usize,
        DUMB_HEAP_SIZE
    );

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

    // We're going to relocate ourselves, grab some memory
    let kernelBytesPhysicalAddress: *mut u8 =
        physicalMemoryManager.ReserveWherever(kernelSize, 0x1000);
    loggerWriteLine!(
        "New kernel home @ 0x{:X} for 0x{:X}",
        kernelBytesPhysicalAddress as usize,
        kernelSize
    );

    let mut virtualMemoryManager = VirtualMemoryManager::new(physicalMemoryManager, pageBook, bdh);
    loggerWriteLine!("VMM created");

    virtualMemoryManager.map(
        kernelBytesPhysicalAddress as usize,
        VM_KERNEL64_CODE,
        kernelSize,
        Execute::Yes,
        Present::Yes,
        Writable::No, // BUGBUG: Flip back after debugging
        Cachable::No,
        UserSupervisor::Supervisor,
        WriteThrough::WriteTrough,
    );

    reloadCR3();

    // Virtual memory address of the entry point into the kernel
    // We load the whole elf file in memory right now so there's stuff before this address
    let newKernelLocation;

    unsafe {
        let currentBase = 0x8000 as usize;
        core::ptr::copy_nonoverlapping(
            currentBase as *const u8,
            VM_KERNEL64_CODE as *mut u8,
            kernelSize,
        );

        newKernelLocation = relocateKernel64(VM_KERNEL64_CODE, kernelSize);
    }

    let currentTextOffset = 0x9000;
    let newKernelLocationCanonical = VirtualMemoryManager::canonicalize(newKernelLocation);
    loggerWriteLine!(
        "New kernel is @ 0x{:X} / 0x{:X}",
        newKernelLocation,
        newKernelLocationCanonical
    );
    let finalTarget = newKernelLocationCanonical - currentTextOffset;
    loggerWriteLine!("After mucking 0x{:X}", finalTarget);
    // Move to our new kernel space
    unsafe {
        asm!(
            "push rbx",
            "lea rbx, [rip]",
            "add rbx, rax", // 3 bytes
            "jmp rbx", // 2 bytes
            "pop rbx", // There is maybe a better way to do this with labels, but we're just trying to jump here in the newly mapped space. This code is at the same offset as the previosu identity mapped code.
            //"pop rax",
            //"mov rbp, {1}", // We're going to temporarily put the stack in the kernel's data area while we relocate it since it's still mapped to the same physical memory that stack was at before
            //"add rbp, rsp",
            //"mov rsp, rbp",
            in("rax") finalTarget + 5,
            //const VirtualMemoryManager::canonicalize(VM_KERNEL64_DATA),
        );
    }

    /*loggerWriteLine!("Seting up heap...");
    let mut heap = DumbHeap::new(memoryMap);
    let count = 100;
    let myAlloc = heap.DoSomething(count);
    loggerWriteLine!("Allocated 0x{:X} at 0x{:X}", count, myAlloc);

    heap.DumpHeap();*/

    // BUGBUG: Having trouble transfering this to the virtual memory manager
    //let pp = &mut physicalMemoryManager as *mut PhysicalMemoryManager;

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
