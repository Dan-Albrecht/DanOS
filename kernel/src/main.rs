#![no_std]
#![no_main]
#![allow(non_snake_case)]

mod a20Stuff;
mod pagingStuff;

use core::arch::asm;
use core::panic::PanicInfo;

use a20Stuff::IsTheA20LineEnabled;
use kernel_shared::memory::map::MemoryMap;
use kernel_shared::relocation::relocateKernel64;
use kernel_shared::assemblyStuff::cpuID::Is64BitModeSupported;
use kernel_shared::assemblyStuff::halt::haltLoop;
use kernel_shared::assemblyStuff::misc::disablePic;
use kernel_shared::gdtStuff::Setup64BitGDT;
use kernel_shared::{haltLoopWithMessage, memory, vgaWriteLine};
use pagingStuff::enablePaging;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vgaWriteLine!("32-bit kernel panic!");
    vgaWriteLine!("{}", info.message());
    haltLoopWithMessage!("{}", info);
}

#[cfg(debug_assertions)]
fn sayHello() {
    vgaWriteLine!("Hi from 32-bit Debug Rust!");
}

#[cfg(not(debug_assertions))]
fn sayHello() {
    vgaWriteLine!("Hi from 32-bit Release Rust!");
}

// Arguments are 32-bit since we know the bootloader code is operating in that mode
// Args in ECX, EDX, then stack
#[unsafe(no_mangle)]
pub extern "fastcall" fn DanMain(
    kernel64Address: u32,
    kernel64Length: u32,
    memoryMapLocation: u32,
) -> ! {
    unsafe {
        sayHello();

        // We don't have the interrupt table setup yet, try and prevent random things from trying to send us there
        disablePic();

        vgaWriteLine!("K64: 0x{:X} K64L: 0x{:X} MM: 0x{:X}", kernel64Address, kernel64Length, memoryMapLocation);
        vgaWriteLine!("Relocating 64-bit kernel...");

        let jumpTarget = relocateKernel64(kernel64Address.try_into().expect("kernel64Address"), kernel64Length.try_into().expect("kernel64Length"));

        vgaWriteLine!("Loading memory map from 0x{:X}", memoryMapLocation);
        // BUGBUG: Want this to be a copy as the memory location this is in probably isn't the best
        let memoryMap = memoryMapLocation as *const MemoryMap;
        let memoryMap = &*memoryMap;
        memoryMap.dump();

        if IsTheA20LineEnabled(&memoryMap) {
            if Is64BitModeSupported() {
                vgaWriteLine!("64-bit mode is available");

                let entry = memoryMap.Entries[0];
                let cantUseAbove = enablePaging(&memoryMap);

                vgaWriteLine!("64-bit paging mode enabled...");
                vgaWriteLine!("...though we're in compatability (32-bit) mode currently.");
                Setup64BitGDT(entry.BaseAddress, cantUseAbove);

                vgaWriteLine!(
                    "The new GDT is in place. Jumping to 64-bit 0x{:X}...",
                    jumpTarget
                );

                // Cannot figure out how to get Rust to emit a long jump with a variable as the address
                // ChatGPT said do a retf instead and that seems to work
                asm!(
                    "mov edi, {0}", // First param for kernel64. https://www.ired.team/miscellaneous-reversing-forensics/windows-kernel-internals/linux-x64-calling-convention-stack-frame
                    "mov esi, {1}",
                    "push 0x8",     // Code segment
                    "push {2}",     // Address in that segment
                    "retf",
                    in(reg) memoryMapLocation,
                    in(reg) kernel64Length,
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
