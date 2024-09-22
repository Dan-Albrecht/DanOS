#![no_std]
#![no_main]
#![allow(non_snake_case)]

mod a20Stuff;
mod pagingStuff;

use core::arch::asm;
use core::panic::PanicInfo;
use core::ptr::{read_unaligned, slice_from_raw_parts, write_unaligned};

use a20Stuff::IsTheA20LineEnabled;
use core::fmt::Write;
use elf::abi::{R_X86_64_RELATIVE, STT_NOTYPE};
use elf::endian::NativeEndian;
use elf::file::Class;
use elf::ElfBytes;
use kernel_shared::assemblyStuff::cpuID::Is64BitModeSupported;
use kernel_shared::assemblyStuff::halt::haltLoop;
use kernel_shared::assemblyStuff::misc::disablePic;
use kernel_shared::gdtStuff::Setup64BitGDT;
use kernel_shared::memoryMap::MemoryMap;
use kernel_shared::{haltLoopWithMessage, vgaWriteLine};
use pagingStuff::enablePaging;

// Returns the offset (relative to base address) of where the .text section is
unsafe fn relocateKernel64(baseAddress: u32, length: u32) -> u32 {
    let length: usize = length.try_into().unwrap();
    vgaWriteLine!("Parsing ELF @ 0x{:X} for 0x{:X}", baseAddress, length);
    let data = slice_from_raw_parts(baseAddress as *const u8, length);
    let elf = ElfBytes::<NativeEndian>::minimal_parse(&*data).expect("ELF parse failed");
    if elf.ehdr.class != Class::ELF64 {
        haltLoopWithMessage!("Expected 64bit elf, got: {:?}", elf.ehdr.class);
    }

    let textSection = elf
        .section_header_by_name(".text")
        .expect("Couldn't parse section table")
        .expect("Couldn't find .text section");
    let textOffset = textSection.sh_offset;
    let kernelThinks = textSection.sh_addr;
    let actualTextLocation: u64 = textOffset + baseAddress as u64;
    let relocationAdjustment = actualTextLocation.wrapping_sub(kernelThinks);

    if actualTextLocation != kernelThinks {
        vgaWriteLine!(
            "Kernel thinks it's @ 0x{:X}, but is @ {:X}",
            kernelThinks,
            actualTextLocation
        );
    }

    let sections = elf.section_headers().expect("No ELF headers...");
    let mut relocationCount = 0;

    for section in sections.iter() {
        if let Ok(relocations) = elf.section_data_as_relas(&section) {
            for relocation in relocations {
                if relocation.r_sym == 0 {
                    // BUGBUG: This library sucks, no idea what constant to use, just know this is 0. Might be STB_LOCAL though the size is wrong.
                    // Good relocation tutorial: https://fasterthanli.me/series/making-our-own-executable-packer/part-17
                    if relocation.r_type == R_X86_64_RELATIVE {
                        let offset = relocation.r_offset;
                        let addend = relocation.r_addend;
                        let result = relocationAdjustment.wrapping_add(addend as u64);
                        let target = offset.wrapping_add(relocationAdjustment) as *mut u64;

                        write_unaligned(target, result);
                        relocationCount += 1;
                    } else {
                        haltLoopWithMessage!(
                            "Don't know how to do a {} relocation",
                            relocation.r_type
                        );
                    }
                } else {
                    haltLoopWithMessage!("Don't know how to relocate a {}", relocation.r_sym);
                }
            }
        }
    }

    vgaWriteLine!(
        "Relocated {} entries. .text offset is 0x{:X}",
        relocationCount,
        textOffset
    );
    return textOffset.try_into().expect("Kernel64 .text offset");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vgaWriteLine!("32-bit kernel panic!");
    vgaWriteLine!("{}", info.message());
    haltLoopWithMessage!("{}", info);
}

// Arguments are 32-bit since we know the bootloader code is operating in that mode
// Args in ECX, EDX, then stack
#[no_mangle]
pub extern "fastcall" fn DanMain(
    kernel64Address: u32,
    kernel64Length: u32,
    memoryMapLocation: u32,
) -> ! {
    unsafe {
        // Previous stage didn't newline after its last message
        vgaWriteLine!("\r\nWe've made it to Rust!");

        // We don't have the interrupt table setup yet, try and prevent random things from trying to send us there
        disablePic();

        vgaWriteLine!("Relocating 64-bit kernel...");
        let textOffset = relocateKernel64(kernel64Address, kernel64Length);

        vgaWriteLine!("Loading memory map from 0x{:X}", memoryMapLocation);
        let memoryMap = MemoryMap::Load(memoryMapLocation.try_into().expect("Memory map"));
        memoryMap.Dump();

        if IsTheA20LineEnabled(&memoryMap) {
            if Is64BitModeSupported() {
                vgaWriteLine!("64-bit mode is available");

                let entry = memoryMap.Entries[0];
                let cantUseAbove = enablePaging(&memoryMap);

                vgaWriteLine!("64-bit paging mode enabled...");
                vgaWriteLine!("...though we're in compatability (32-bit) mode currently.");
                Setup64BitGDT(entry.BaseAddr, cantUseAbove);

                let jumpTarget = kernel64Address + textOffset;

                vgaWriteLine!(
                    "The new GDT is in place. Jumping to 64-bit 0x{:X}...",
                    jumpTarget
                );

                // Cannot figure out how to get Rust to emit a long jump with a variable as the address
                // ChatGPT said do a retf instead and that seems to work
                asm!(
                    "mov edi, {0}", // First param for kernel64. https://www.ired.team/miscellaneous-reversing-forensics/windows-kernel-internals/linux-x64-calling-convention-stack-frame
                    "push 0x8",     // Code segment
                    "push {1}",     // Address in that segment
                    "retf",
                    in(reg) memoryMapLocation,
                    in(reg) jumpTarget,
                );

                vgaWriteLine!("64-bit kernel returned!");
            } else {
                vgaWriteLine!("No 64-bit mode. :(");
            }
        } else {
            vgaWriteLine!("You have hardware/emulator with the A20 address line disabled...");
        }
    };

    haltLoop();
}
