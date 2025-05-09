#![allow(non_snake_case)]
#![cfg(target_pointer_width = "32")]

use core::arch::asm;

const EXTENDED_FUNCTION_INFORMATION: u32 = 0x80000000;
const EXTENDED_PROCESSOR_INFO: u32 = 0x80000001;

// BUGBUG: The loaded assembly is screwed up, its only because we run through a ton of 0x0 instructions
// we even make it here.
unsafe fn IsCpuIDSupported() -> bool { unsafe {
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
}}

unsafe fn AreExtendedCpuIDFunctionsSupported() -> bool { unsafe {
    if !IsCpuIDSupported() {
        return false;
    }

    let cpuid = CpuId(EXTENDED_FUNCTION_INFORMATION);

    // If this is bigger than
    if cpuid.eax > EXTENDED_FUNCTION_INFORMATION {
        true
    } else {
        false
    }
}}

pub unsafe fn Is64BitModeSupported() -> bool { unsafe {
    if !AreExtendedCpuIDFunctionsSupported() {
        return false;
    }

    let cpuid = CpuId(EXTENDED_PROCESSOR_INFO);

    // 29th bit says if this is supported or not
    if (cpuid.edx & (1 << 29)) != 0 {
        return true;
    } else {
        return false;
    }
}}

// NB: Clobbers vary by function. The clobbers below are just for the two functions I'm using here.
fn CpuId(function: u32) -> CpuIdResult {
    unsafe {
        let (mut eax, mut ebx, mut ecx, mut edx): (u32, u32, u32, u32);
        eax = function;
        asm!(
            // LLVM doesn't want us messing with bx, so
            // indirect through edi
            "push ebx",
            "cpuid",
            "mov edi, ebx",
            "pop ebx",
            inout("eax") eax,
            out("edi") ebx,
            out("ecx") ecx,
            out("edx") edx,
        );

        return CpuIdResult {
            eax,
            _ebx: ebx,
            _ecx: ecx,
            edx,
        };
    }
}

struct CpuIdResult {
    eax: u32,
    _ebx: u32,
    _ecx: u32,
    edx: u32,
}
