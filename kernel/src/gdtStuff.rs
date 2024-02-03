// Global Descriptor Table

use core::{arch::asm, ptr::addr_of};

#[repr(C, packed)]
struct OurGdt {
    nullSection: u64,
    codeSection: u64,
    dataSection: u64,
    gdtSize: u16,
    selfPointer: u64,
}

pub unsafe fn Setup64BitGDT() {

    let mut ourGdt = OurGdt {
        nullSection: 0,
        codeSection: 
            1 << (32 + 21)      /* Long Mode - 64bit */ 
            | 1 << (32 + 15)    /* Present */ 
            // 13 & 14 = 0. DPL - This is for Ring0
            | 1 << (32 + 12)    /* S Field - User Descriptor */ 
            | 1 << (32 + 11)    /* Code/Data - Code Segment */ 
            | 1 << (32 + 10),   /* Conforming - */ 
        dataSection:
            1 << (32 + 21)      /* Long Mode - 64bit */ 
            | 1 << (32 + 15)    /* Present */
            // 13 & 14 = 0. DPL - This is for Ring0
            | 1 << (32 + 12)    /* S Field - User Descriptor */ 
            /* 11 = 0. Data segment */
            | 1 << (32 + 9),    /* Writable */ 
        gdtSize: (core::mem::size_of::<OurGdt>() as u16) - 1, // BUGBUG: OS wiki loves to say -1, but don't see that in AMD manual
        selfPointer: 0,
    };

    ourGdt.selfPointer = addr_of!(ourGdt) as u64;

    asm!(
        "add eax, 0x18", // BUGBUG: Find a better way
        "lgdt [eax]",
        in("eax") addr_of!(ourGdt)
    );
}
