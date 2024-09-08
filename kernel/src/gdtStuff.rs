// Global Descriptor Table

use core::arch::asm;

use kernel_shared::{magicConstants::GDT_ADDRESS, memoryHelpers::zeroMemory2};

#[repr(C, packed)]
struct OurGdt {
    nullSection: u64,
    codeSection: u64,
    dataSection: u64,
    gdtSize: u16,
    selfPointer: u64,
}

pub unsafe fn Setup64BitGDT() {

    let ourGdt = GDT_ADDRESS as *mut OurGdt;
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
