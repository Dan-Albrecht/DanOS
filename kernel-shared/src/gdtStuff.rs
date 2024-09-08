// Global Descriptor Table

use core::arch::asm;
use core::fmt::Write;

use crate::memoryHelpers::{alignDown, zeroMemory2};
use crate::{haltLoopWithMessage, vgaWriteLine};

use crate::assemblyStuff::halt::haltLoop;

const GDT_ALIGNMENT : usize = 0x10;

#[repr(C, packed)]
struct OurGdt {
    nullSection: u64,
    codeSection: u64,
    dataSection: u64,
    gdtSize: u16,
    selfPointer: u64,
}

pub unsafe fn Setup64BitGDT(baseAddress: u64, cantUseAbove: usize) {

    let gdtAddress = alignDown(cantUseAbove - 1 - size_of::<OurGdt>(), GDT_ALIGNMENT);
    let baseAddress = baseAddress as usize;

    if gdtAddress < baseAddress {
        haltLoopWithMessage!("Can't put GDT @ 0x{:X}", gdtAddress);
    } else {
        vgaWriteLine!("Putting GDT @ 0x{:X}", gdtAddress);
    }

    let ourGdt = gdtAddress as *mut OurGdt;
    zeroMemory2(ourGdt);

    (*ourGdt).codeSection = 
            1 << (32 + 21)      /* Long Mode - 64bit */ 
            | 1 << (32 + 15)    /* Present */ 
            // 13 & 14 = 0. DPL - This is for Ring0
            | 1 << (32 + 12)    /* S Field - User Descriptor */ 
            | 1 << (32 + 11)    /* Code/Data - Code Segment */ 
            | 1 << (32 + 10);   /* Conforming - */ 

    (*ourGdt).dataSection = 
            1 << (32 + 21)      /* Long Mode - 64bit */ 
            | 1 << (32 + 15)    /* Present */
            // 13 & 14 = 0. DPL - This is for Ring0
            | 1 << (32 + 12)    /* S Field - User Descriptor */ 
            /* 11 = 0. Data segment */
            | 1 << (32 + 9);    /* Writable */ 

    (*ourGdt).gdtSize = 
            (core::mem::size_of::<OurGdt>() as u16) - 1; // BUGBUG: OS wiki loves to say -1, but don't see that in AMD manual

    (*ourGdt).selfPointer = ourGdt as u64;

    asm!(
        "add eax, 0x18", // BUGBUG: Find a better way
        "lgdt [eax]",
        in("eax") ourGdt
    );
}

#[repr(C, packed)]
struct GdtrInternal {
    Length: u16,
    BaseAddress: u64,
}

pub struct Gdtr {
    pub BaseAddress: u64,
    pub Length: u16,
}

#[cfg(target_pointer_width = "64")]
pub fn GetGdtr() -> Gdtr {
    let gdtr = GdtrInternal{BaseAddress: 0,Length: 0};

    unsafe {
        // https://www.felixcloutier.com/x86/sgdt
        asm!(
            "sgdt [{}]",
            in(reg) &gdtr,
            options(nostack, preserves_flags),
        );
    }

    let limit = gdtr.Length;
    let base = gdtr.BaseAddress;

    Gdtr{
        BaseAddress :base,
        Length:limit,
    }
}
