#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![feature(log_syntax)]
#![feature(cfg_relocation_model)]

mod disk;

use core::{arch::asm, panic::PanicInfo};
use disk::{
    diskDriver::DiskDriver,
    fatDriver::FatDriver,
};
use kernel_shared::{
    assemblyStuff::{halt::haltLoop, misc::disablePic}, gdtStuff::Gdt, haltLoopWithMessage, magicConstants::KERNEL32_JUMP_ADDRESS, memory::{map::MemoryMap, mapEntry::MemoryMapEntryType}, textMode::teletype, vgaWriteLine
};

#[panic_handler]
fn panic(pi: &PanicInfo) -> ! {
    // BUGBUG: Seems the VGA write methods don't update the cursor position in a way the BIOS functions will notice
    // so just pick one of the two methods to use for now
    if let Some(msg) = pi.message().as_str() {
        teletype::printLine(b"16-bit panic!");
        teletype::printLine(msg.as_bytes());
        teletype::printLine(b"End of line");

        unsafe {
            loop {
                asm!("hlt");
            }
        }
    } else {
        vgaWriteLine!("16-big panic!");
        // We're risking a further panic here, but really want to see the message
        haltLoopWithMessage!("{:?}", pi);
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

fn whereAreWe() -> usize {
    let mut eip: u32;
    unsafe {
        asm!(
            "call 2f",
            "2:",
            "pop {}",
            out(reg) eip
        );
    }

    eip as usize
}

fn findSpaceForKernels(mm: &MemoryMap, needed_size: usize) -> usize {
    // Figure out, roughly, where we are in memory.
    // We'll just assume whatever entry we're in is fully used for now and skip it.
    let eip = whereAreWe();
    vgaWriteLine!("EIP is at 0x{:X}", eip);

    for region in mm.Entries.iter() {
        if region.getType() == MemoryMapEntryType::AddressRangeMemory {
            if region.BaseAddress as usize <= eip
                && eip < (region.BaseAddress + region.Length) as usize
            {
                vgaWriteLine!(
                    "Skipping region we're currently in: 0x{:X} - 0x{:X}",
                    { region.BaseAddress },
                    { region.BaseAddress + region.Length }
                );
                continue;
            }

            if region.Length >= needed_size as u64 {
                vgaWriteLine!(
                    "Kernel32 will fit in region: 0x{:X} - 0x{:X}",
                    { region.BaseAddress },
                    { region.BaseAddress + region.Length }
                );

                if region.BaseAddress != KERNEL32_JUMP_ADDRESS as u64 {
                    // BUGBUG:
                    // 1) Handle ELF files and relocation
                    // 2) This address might bein the region, it's just the region didn't exactly start there
                    haltLoopWithMessage!("Kernel32 jump address is not where we expected it to be");
                }

                return region.BaseAddress as usize;
            }
        }
    }

    haltLoopWithMessage!("Failed to find space for kernel32");
}

#[unsafe(no_mangle)]
pub extern "fastcall" fn DanMain(driveNumber: u32) -> ! {
    #[cfg(not(relocation_model = "static"))]
    compile_error!("Stage1 boot loader cannot handle having to relocate us.");

    disablePic();

    // We need full 32-bit segment offsets to access everything as this code
    // doesn't compile in a way that it knows to manipulate the segment registers.
    // Only static strings should be used before this switch as fmt loves to
    // try and jump somwhere we cannot yet reach.
    let gdt = Gdt::create32BitFlat();
    unsafe {
        gdt.enterUnrealMode();
    };

    sayHello();

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

    let fat = FatDriver::new(disk).unwrap();
    fat.printHiText().unwrap();

    let kernel32Info = fat.getFileInfo((b"KERNEL", b"BIN")).unwrap();
    let kernel32PaddedSize = ((kernel32Info.file_size as usize + 1023) / 1024) * 1024;

    let kernel64Info = fat.getFileInfo((b"KERNEL64", b"ELF")).unwrap();
    let kernel64PaddedSize = ((kernel64Info.file_size as usize + 1023) / 1024) * 1024;

    let kernel32Address = findSpaceForKernels(&mm, kernel32PaddedSize + kernel64PaddedSize);
    let kernel64Address = kernel32Address + kernel32PaddedSize;

    unsafe {
        fat.loadFile(kernel32Address, &kernel32Info).unwrap();
        fat.loadFile(kernel64Address, &kernel64Info).unwrap();
    }

    vgaWriteLine!(
        "Loaded kernel32 to 0x{:X} for {} bytes",
        kernel32Address,
        kernel32Info.file_size
    );

    vgaWriteLine!(
        "Loaded kernel64 to 0x{:X} for {} bytes",
        kernel64Address,
        kernel64Info.file_size
    );

    unsafe {
        gdt.enterProtectedMode(KERNEL32_JUMP_ADDRESS, kernel64Address, kernel64PaddedSize, kernel32Address, kernel32PaddedSize, &mm as *const _ as usize);
    }
}
