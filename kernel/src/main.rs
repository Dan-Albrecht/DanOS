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
use kernel_shared::{haltLoopWithMessage, loggerWriteLine};
use pagingStuff::enablePaging;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loggerWriteLine!("32-bit kernel panic!");
    loggerWriteLine!("{}", info.message());
    haltLoopWithMessage!("{}", info);
}

#[cfg(debug_assertions)]
fn sayHello() {
    loggerWriteLine!("Hi from 32-bit Debug Rust!");
}

#[cfg(not(debug_assertions))]
fn sayHello() {
    loggerWriteLine!("Hi from 32-bit Release Rust!");
}

// Arguments are 32-bit since we know the bootloader code is operating in that mode
// Args in ECX, EDX, then stack
#[unsafe(no_mangle)]
pub extern "fastcall" fn DanMain(
    kernel64Address: u32,
    kernel64Length: u32,
    kernel32Address: u32,
    kernel32Length: u32,
    memoryMapLocation: u32,
) -> ! {
    unsafe {
        sayHello();

        // We don't have the interrupt table setup yet, try and prevent random things from trying to send us there
        disablePic();

        loggerWriteLine!("Stage3 - K64: 0x{:X} K64L: 0x{:X} K32: 0x{:X} K32L: 0x{:X} MM: 0x{:X}", kernel64Address, kernel64Length, kernel32Address, kernel32Length, memoryMapLocation);
        loggerWriteLine!("Relocating 64-bit kernel...");

        let jumpTarget = relocateKernel64(kernel64Address.try_into().expect("kernel64Address"), kernel64Length.try_into().expect("kernel64Length"));

        loggerWriteLine!("Loading memory map from 0x{:X}", memoryMapLocation);
        // BUGBUG: Want this to be a copy as the memory location this is in probably isn't the best
        let memoryMap = memoryMapLocation as *const MemoryMap;
        let memoryMap = &*memoryMap;
        memoryMap.dumpEx(true);

        if IsTheA20LineEnabled(&memoryMap) {
            if Is64BitModeSupported() {
                loggerWriteLine!("64-bit mode is available");

                let entry = memoryMap.Entries[0];
                let pageTableData = enablePaging(&memoryMap);

                loggerWriteLine!("64-bit paging mode enabled...");
                loggerWriteLine!("...though we're in compatability (32-bit) mode currently.");
                loggerWriteLine!("Page table data is between 0x{:X} and 0x{:X}", pageTableData.PageStructuresStart, pageTableData.PageStructuresEnd);
                let gdtAddress = Setup64BitGDT(entry.BaseAddress, pageTableData.PageStructuresStart);

                loggerWriteLine!(
                    "The new GDT is in place. Jumping to 64-bit 0x{:X}...",
                    jumpTarget
                );

                // Cannot figure out how to get Rust to emit a long jump with a variable as the address
                // ChatGPT said do a retf instead and that seems to work
                asm!(
                    // First param for kernel64. https://www.ired.team/miscellaneous-reversing-forensics/windows-kernel-internals/linux-x64-calling-convention-stack-frame
                    "mov  esi, eax", // Rust won't let us directly set this, so need to do it indirectly
                    "push 0x8",      // Code segment
                    "push ebx",      // Address in that segment
                    "retf",
                    in("edi") memoryMapLocation,
                    in("eax") kernel64Address,
                    in("edx") kernel64Length,
                    in("ebx") jumpTarget,
                    in("ecx") gdtAddress,
                );

                loggerWriteLine!("64-bit kernel returned!");
            } else {
                loggerWriteLine!("No 64-bit mode. :(");
            }
        } else {
            loggerWriteLine!("You have hardware/emulator with the A20 address line disabled...");
        }
    };

    haltLoop();
}
