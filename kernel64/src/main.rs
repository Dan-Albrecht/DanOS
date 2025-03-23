#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![feature(naked_functions)]
#![feature(abi_x86_interrupt)]
#![feature(used_with_arg)]
#![feature(concat_idents)]
#![feature(const_trait_impl)]
#![feature(if_let_guard)]
#![feature(core_intrinsics)]

mod acpi;
mod ahci;
mod assemblyHelpers;
mod diskStuff;
mod interupts;
mod magicConstants;
mod memory;

use core::arch::asm;
use core::array::from_fn;
use core::panic::PanicInfo;

use interupts::InteruptDescriptorTable::{IDT, SetIDT};

use kernel_shared::gdtStuff::{GDTR, Gdt, GetGdtr};
use kernel_shared::memory::map::MemoryMap;
use kernel_shared::memoryTypes::{PhysicalAddress, VirtualAddress};
use kernel_shared::pageTable::enums::*;
use kernel_shared::pageTable::pageMapLevel4Table::PageMapLevel4Table;
use kernel_shared::physicalMemory::{MemoryBlob, PhysicalMemoryManager, WhatDo};
use kernel_shared::relocation::relocateKernel64;
use kernel_shared::{
    assemblyStuff::{halt::haltLoop, misc::Breakpoint},
    pageTable::pageBook::PageBook,
};
use kernel_shared::{haltLoopWithMessage, loggerWriteLine, magicConstants::*};
use magicConstants::*;
use memory::dumbHeap::BootstrapDumbHeap;
use memory::virtualMemory::VirtualMemoryManager;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // We can get called mid-line, so always move to a new one
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

fn mapKernelCode(
    virtualMemoryManager: &mut VirtualMemoryManager,
    kernelBytesPhysicalAddress: usize,
    kernelSize: usize,
) {
    virtualMemoryManager.map(
        kernelBytesPhysicalAddress,
        VM_KERNEL64_CODE,
        kernelSize,
        Execute::Yes,
        Present::Yes,
        Writable::No,
        Cachable::No,
        UserSupervisor::Supervisor,
        WriteThrough::WriteTrough,
    );
}

fn mapKernelData(
    virtualMemoryManager: &mut VirtualMemoryManager,
    kernelStackPhysicalAddress: usize,
) {
    virtualMemoryManager.map(
        kernelStackPhysicalAddress,
        VM_KERNEL64_DATA,
        VM_KERNEL64_DATA_LENGTH,
        Execute::Yes, // BUGBUG: Something is really screwed up, we're page faulint if this isn't executable...but its stack space...
        Present::Yes,
        Writable::Yes,
        Cachable::No,
        UserSupervisor::Supervisor,
        WriteThrough::WriteTrough,
    );
}

// Arguments 1-6 are passed via registers RDI, RSI, RDX, RCX, R8, R9 respectively;
// Arguments 7 and above are pushed on to the stack.
#[unsafe(no_mangle)]
pub extern "sysv64" fn DanMain(
    memoryMapLocation: usize,
    kernelElfLocation: usize,
    kernelElfSize: usize,
) -> ! {
    loggerWriteLine!(
        "Welcome to 64-bit Rust! We're 0x{:X} bytes long starting at 0x{:X}. Memory map is at 0x{:X}",
        kernelElfSize,
        kernelElfLocation,
        memoryMapLocation
    );

    haltLoop();

    let memoryMap: MemoryMap;
    unsafe {
        memoryMap = *(memoryMapLocation as *const MemoryMap);
    }

    memoryMap.dump();

    let mut physicalMemoryManager = PhysicalMemoryManager {
        MemoryMap: memoryMap,
        Blobs: from_fn(|_| MemoryBlob::default()),
    };

    // BUGBUG: Should probalby get the base pointer as this function has already subtracted stack space
    let sp = getSP();
    // BUGBUG: Get a proper stack size
    loggerWriteLine!("Reserving stack 0x{:X}", sp);
    physicalMemoryManager.Reserve(sp, 1, WhatDo::YoLo);

    // Reserve ourself
    loggerWriteLine!("Reserving self 0x{:X}", kernelElfLocation);
    physicalMemoryManager.Reserve(kernelElfLocation, kernelElfSize, WhatDo::Normal);

    // This is probably not in the memory map, but if it shows up, we want to mark it as used
    loggerWriteLine!("Reserving VGA 0x{:X}", VGA_BUFFER_ADDRESS);
    physicalMemoryManager.Reserve(
        VGA_BUFFER_ADDRESS.try_into().unwrap(),
        (VGA_WIDTH * VGA_HEIGHT * VGA_BYTES_PER_CHAR).into(),
        WhatDo::YoLo,
    );

    loggerWriteLine!("Installing interrupt table...");
    unsafe {
        SetIDT(&mut physicalMemoryManager);
    }

    loggerWriteLine!("Sending a breakpoint...");
    Breakpoint();
    loggerWriteLine!("We handled the breakpoint!");

    const DUMB_HEAP_SIZE: usize = 0x5_0000;
    let dumbHeapAddress: *mut u8 = physicalMemoryManager.ReserveWhereverZeroed(DUMB_HEAP_SIZE, 1);
    loggerWriteLine!(
        "Dumb heap @ 0x{:X} for 0x{:X}",
        dumbHeapAddress as usize,
        DUMB_HEAP_SIZE
    );

    let pageBook = PageBook::fromExistingIdentityMapped();
    // This is using identity mapping, so nothing to adjust
    let bdh = BootstrapDumbHeap::new(dumbHeapAddress as usize, DUMB_HEAP_SIZE, false, 0);
    loggerWriteLine!("PageBook @ 0x{:X}", pageBook.getCR3Value() as usize);

    // We're going to relocate ourselves, grab some memory
    let kernelBytesPhysicalAddress: *mut u8 =
        physicalMemoryManager.ReserveWhereverZeroed(kernelElfSize, 0x1000);
    let kernelStackPhysicalAddress: *mut u8 =
        physicalMemoryManager.ReserveWhereverZeroed(VM_KERNEL64_DATA_LENGTH, 0x1000);
    loggerWriteLine!(
        "New kernel home @ (P) 0x{:X} for 0x{:X}",
        kernelBytesPhysicalAddress as usize,
        kernelElfSize
    );

    let mut virtualMemoryManager = VirtualMemoryManager::new(physicalMemoryManager, pageBook, bdh);
    loggerWriteLine!("VMM created");

    mapKernelCode(
        &mut virtualMemoryManager,
        kernelBytesPhysicalAddress as usize,
        kernelElfSize,
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
            kernelElfSize,
        );

        newKernelLocation = relocateKernel64(VM_KERNEL64_CODE, kernelElfSize);
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
            in("rax") finalTarget + 5,
        );
    }

    loggerWriteLine!("Kernel code has been relocated, now to stack...");

    mapKernelData(
        &mut virtualMemoryManager,
        kernelStackPhysicalAddress as usize,
    );
    reloadCR3();

    // Stack grows down, so put it at the end of the space
    let stackTarget =
        VirtualMemoryManager::canonicalize(VM_KERNEL64_DATA + VM_KERNEL64_STACK_LENGTH);

    unsafe {
        asm!(
            "mov rsp, rax",
            "mov rbp, r9",
            "jmp r9",
            in("rax") stackTarget,
            in("r9") newStackHome as usize,
            in("rdi") memoryMapLocation,
            in("rsi") kernelBytesPhysicalAddress,
            in("rdx") kernelElfSize,
            in("rcx") kernelStackPhysicalAddress,
        );
    }

    unreachable!("Retunred from new stack!");
}

// This currently ends up at 0x2130d0
// VM_KERNEL64_CODE (0x20_0000) + (.text offset) 0x1000 + (function offset) 0x120d0
extern "sysv64" fn newStackHome(
    memoryMapLocation: usize,
    kernelCodePhysical: usize,
    kernelCodeLength: usize,
    kernelDataPhysical: usize,
) -> ! {
    loggerWriteLine!("We are using our new stack space");
    haltLoopWithMessage!("Temp parking");

    // The memoryMapLocation is in a location we're about to unmap and/or repurpose, so copy its data and never use it again
    //let memoryMap = MemoryMap::Load(memoryMapLocation);
    // BUGBUG: Reload this
    //let memoryMap = MemoryMap::Load(memoryMapLocation.try_into().unwrap());
    let memoryMap: MemoryMap;
    unsafe {
        memoryMap = *(memoryMapLocation as *const MemoryMap);
    }

    let mut physicalMemoryManager = PhysicalMemoryManager {
        MemoryMap: memoryMap,
        Blobs: from_fn(|_| MemoryBlob::default()),
    };

    physicalMemoryManager.Dump();
    physicalMemoryManager.Reserve(kernelCodePhysical, kernelCodeLength, WhatDo::Normal);
    physicalMemoryManager.Reserve(kernelDataPhysical, VM_KERNEL64_DATA_LENGTH, WhatDo::Normal);

    // BUGBUG: Magic constant
    const DUMB_HEAP_SIZE: usize = 0x5_0000;

    // We're in the course of setting up a new virtual memory manager. We're currently executing in non-identity mapped space
    // so we cannot just ask the physical manager for unused space. We know nothing has used the kernel data space yet aside
    // from the stack, so just take space next to it and then we'll tell the virtual manager about it after it is up.
    let bdhAddress = VM_KERNEL64_DATA + VM_KERNEL64_STACK_LENGTH;
    let adjustment = VM_KERNEL64_DATA - kernelDataPhysical;
    let mut bdh = BootstrapDumbHeap::new(bdhAddress, DUMB_HEAP_SIZE, true, adjustment);

    // BUGBUG: BDH alocates from data space. Potential one of the reason we had to mark that executable...
    loggerWriteLine!("Installing new interrupt table...");
    let _idt = IDT::new(&mut bdh);

    loggerWriteLine!("Sending a breakpoint...");
    Breakpoint();
    loggerWriteLine!("We handled the new breakpoint!");

    // BUGBUG: This is on the stack, we should probably allocate from BDH
    let gdt = Gdt::new();
    let mut gdtr = GDTR::new();
    loggerWriteLine!("GDT @ 0x{:X}", &gdt as *const _ as usize);
    unsafe {
        gdtr.install(gdt);
    }

    // BUGBUG: More stack stuff
    let pml4 = PageMapLevel4Table::new();
    let vir = &pml4 as *const _ as usize;
    let phs = vir - adjustment;
    let pageBook = PageBook::new(
        false,
        false,
        PhysicalAddress::<PageMapLevel4Table>::new(phs),
        VirtualAddress::<PageMapLevel4Table>::new(vir),
    );
    loggerWriteLine!(
        "PageBook @ 0x{:X}, BDH @ 0x{:X}",
        pageBook.getCR3Value() as usize,
        bdhAddress
    );
    let cr3 = pageBook.getCR3Value();
    let cr3P = pageBook.getPhysical();
    let cr3V = pageBook.getVirtual();

    // Create new VM map. This will get rid of the identity map we previously had.
    let mut virtualMemoryManager = VirtualMemoryManager::new(physicalMemoryManager, pageBook, bdh);
    mapKernelCode(
        &mut virtualMemoryManager,
        kernelCodePhysical,
        kernelCodeLength,
    );

    mapKernelData(&mut virtualMemoryManager, kernelDataPhysical as usize);

    virtualMemoryManager.identityMap(
        VGA_BUFFER_ADDRESS.try_into().unwrap(),
        (VGA_WIDTH * VGA_HEIGHT * VGA_BYTES_PER_CHAR).into(),
        Execute::Yes, // BUGBUG: Another permissions mystery. Why must this be set? PageFaults if no...
        Present::Yes,
        Writable::Yes,
        Cachable::No,
        UserSupervisor::Supervisor,
        WriteThrough::WriteTrough,
    );

    loggerWriteLine!(
        "New cr is 0x{:X}, 0x{:X} / 0x{:X} (P/V)",
        cr3,
        cr3P.address,
        cr3V.address
    );

    unsafe {
        asm!(
            "mov cr3, rax",
            in("rax") cr3,
        );
    }

    loggerWriteLine!("We're fully remapped");

    //virtualMemoryManager.getFreeVirtualAddress(1);
    //readBytes(&mut virtualMemoryManager);
    haltLoop();
}
