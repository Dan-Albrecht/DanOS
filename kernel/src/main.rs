#![no_std]
#![no_main]
#![allow(non_snake_case)]

mod a20Stuff;
mod pagingStuff;

use core::arch::asm;
use core::panic::PanicInfo;
use core::ptr::slice_from_raw_parts;

use a20Stuff::IsTheA20LineEnabled;
use core::fmt::Write;
use elf::abi::R_X86_64_RELATIVE;
use elf::endian::NativeEndian;
use elf::file::Class;
use elf::ElfBytes;
use kernel_shared::assemblyStuff::cpuID::Is64BitModeSupported;
use kernel_shared::assemblyStuff::halt::haltLoop;
use kernel_shared::assemblyStuff::misc::disablePic;
use kernel_shared::gdtStuff::Setup64BitGDT;
use kernel_shared::magicConstants::MEMORY_MAP_LOCATION;
use kernel_shared::memoryMap::MemoryMap;
use kernel_shared::{haltLoopWithMessage, vgaWriteLine};
use pagingStuff::enablePaging;

const BOGUS_KERNEL_ADDRESS: u32 = 0x1_2345;
const KERNEL64_JUMP_ADDRESS: u32 = getKernel64Address();

const fn getKernel64Address() -> u32 {
    let value = core::option_env!("KERNEL64_JUMP_ADDRESS");
    let mut result: u32 = 0;

    if let Some(theString) = value {
        let bytes = theString.as_bytes();
        let len = bytes.len();

        if len < 3 || bytes[0] != b'0' || bytes[1] != b'x' {
            assert!(
                false,
                "Load address string must be at least 3 characters and start with a 0x prefix"
            );
        }

        let mut pos = 2;

        while pos < len {
            let byte = bytes[pos];
            result <<= 4;

            if byte >= b'0' && byte <= b'9' {
                result += (byte as u32) - (b'0' as u32);
            } else if byte >= b'A' && byte <= b'F' {
                result += 10 + (byte as u32) - (b'A' as u32);
            } else {
                assert!(false, "Invalid character in address string. Hex characters must be in uppercase if you're using them.");
            }
            pos += 1;
        }
    } else {
        // Hardcoding a default so we can compile without having to worry to set this
        // Build scripts will always set this, we'll panic if we see the default value
        result = BOGUS_KERNEL_ADDRESS;
    }

    result
}

unsafe fn relocateKernel64(baseAddress: usize, length: usize) {
    let data = slice_from_raw_parts(baseAddress as *const u8, length);
    let elf = ElfBytes::<NativeEndian>::minimal_parse(&*data).expect("ELF parse failed");
    if elf.ehdr.class != Class::ELF64 {
        haltLoopWithMessage!("Expected 64bit elf, got: {:?}", elf.ehdr.class);
    }
    let sections = elf.section_headers().expect("No ELF headers...");
    let mut relocationCount = 0;

    for section in sections.iter() {
        if let Ok(relocations) = elf.section_data_as_relas(&section) {
            for relocation in relocations {
                if relocation.r_type == R_X86_64_RELATIVE {
                    let ptr = relocation.r_offset as *mut i64;
                    *ptr = relocation.r_addend;
                    relocationCount += 1;
                } else {
                    haltLoopWithMessage!("Don't know how to relocate a {}", relocation.r_type);
                }
            }
        }
    }

    vgaWriteLine!("Relocated {} entries", relocationCount);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vgaWriteLine!("32-bit kernel panic!");
    haltLoopWithMessage!("{info}");
}

#[no_mangle]
pub extern "C" fn DanMain() -> ! {
    unsafe {
        // Previous stage didn't newline after its last message
        vgaWriteLine!("\r\nWe've made it to Rust!");

        if KERNEL64_JUMP_ADDRESS == BOGUS_KERNEL_ADDRESS {
            // BUGBUG: I don't think I still fuly understand const. When coding in VSCode and the env variable
            // is unset, we should be getting flagged about unrearchable code as the else will never hit...
            haltLoopWithMessage!("KERNEL64_JUMP_ADDRESS wasn't set at build time");
        }

        // We don't have the interrupt table setup yet, try and prevent random things from trying to send us there
        disablePic();

        vgaWriteLine!("Relocating 64-bit kernel...");
        relocateKernel64(0x8000, 0x5_F5A0);

        let memoryMap = MemoryMap::Load(MEMORY_MAP_LOCATION);
        memoryMap.Dump();

        if IsTheA20LineEnabled(&memoryMap) {
            if Is64BitModeSupported() {
                vgaWriteLine!("64-bit mode is available");

                let entry = memoryMap.Entries[0];
                let cantUseAbove = enablePaging(&memoryMap);

                vgaWriteLine!("64-bit paging mode enabled...");
                vgaWriteLine!("...though we're in compatability (32-bit) mode currently.");
                Setup64BitGDT(entry.BaseAddr, cantUseAbove);

                vgaWriteLine!(
                    "The new GDT is in place. Jumping to 64-bit 0x{:X}...",
                    KERNEL64_JUMP_ADDRESS
                );

                asm!(
                    "jmp 0x8, {adr}", // Far jump to the 64bit kernel
                    adr = const { KERNEL64_JUMP_ADDRESS },
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
