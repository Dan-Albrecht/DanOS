#![allow(non_snake_case)]

use core::arch::asm;

const MAXIMUM_EXTENDED_FUNCTION: u32 = 0x80000000;
const SOME_EXTENDED_FUNCTION: u32 = 0x80000001;

// BUGBUG: The loaded assembly is screwed up, its only because we run through a ton of 0x0 instructions
// we even make it here.
unsafe fn IsCpuIDSupported() -> bool {
    let eax: u32;

    // CPUID is detected by being ableto flip bit 21 of the EFLAGS register
    // https://wiki.osdev.org/Setting_Up_Long_Mode
    // https://c9x.me/x86/html/file_module_x86_id_45.html
    asm!(
        "pushfd",           // Push flags to the stack so we can get them
        "pop eax",          // Move it to eax
        "mov ecx, eax",     // Create a backup copy so we can clean up after ourselves
        "xor eax, 1 << 21", // Flip the 21 bit
        "push eax",
        "popfd",            // Set flags with the flipped bit
        "pushfd",
        "pop eax",          // Read it back so we can so if it stuck
        "push ecx",
        "popfd",            // Put it back how we found it
        "xor eax, ecx",     // See if the change stuck or not
        out("eax") eax,
        out("ecx") _,
    );

    // The 'modified' version was the same as the original, so the change
    // didn't stick and CPUID isn't supported.
    if eax == 0 {
        return false;
    } else {
        return true;
    }
}

unsafe fn AreExtendedCpuIDFunctionsSupported() -> bool {
    if !IsCpuIDSupported() {
        return false;
    }

    let mut eax: u32 = MAXIMUM_EXTENDED_FUNCTION;
    asm!(
        // Ask for the maximum extended function
        "cpuid",
        inout("eax") eax
    );

    // If this is bigger than
    if eax > MAXIMUM_EXTENDED_FUNCTION {
        true
    } else {
        false
    }
}

pub unsafe fn Is64BitModeSupported() -> bool {
    if !AreExtendedCpuIDFunctionsSupported() {
        return false;
    }

    let mut edx: u32;
    asm!(
        "cpuid",
        in("eax") SOME_EXTENDED_FUNCTION,
        out("edx") edx,
    );

    // 29th bit says if this is supported or not
    if (edx & (1 << 29)) != 0 {
        return true;
    } else {
        return false;
    }
}
