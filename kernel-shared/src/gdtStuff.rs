// Global Descriptor Table

use core::arch::asm;

use crate::memoryHelpers::{alignDown, zeroMemory2};
use crate::{haltLoopWithMessage, loggerWriteLine, vgaWriteLine};
use crate::assemblyStuff::halt::haltLoop;

const GDT_ALIGNMENT : usize = 0x10;

// 3.5.1 Segment Descriptor Tables
// Global Descriptor Table
#[repr(C, align(0x8))]
pub struct Gdt {
    nullSection: u64,
    codeSection: u64,
    dataSection: u64,
}

// 2.4.1 Global Descriptor Table Register (GDTR)
#[repr(C, packed)]
#[cfg(target_pointer_width = "32")]
struct GdtLocator {
    pub Length: u16,
    //pub BaseAddress: u64,
    pub BaseAddress: *const Gdt,
}

impl Gdt {
    pub fn create32BitFlat() -> Self {

        // Figure 3-8. Segment Descriptor
        // The +32 is to just make it easier to match against the figure for the upper half
        let dataSection = 
            0xFFFF << 0      // Limit 15:0
            | 0x0 << 16      // Base 15:0
            | 0x0 << (32+0)  // Base 23:16
            | 0x2 << (32+8)  // Type - W Read/Write
            | 0x1 << (32+12) // S Field - Appliation Descriptor (Code or Data)
            | 0x0 << (32+13) // DPL - Ring 0
            | 0x1 << (32+15) // Present
            | 0xF << (32+16) // Limit 19:16
            | 0x0 << (32+20) // Free for use by system software - we don't need it
            | 0x0 << (32+21) // L - Only 32-bit for now
            | 0x1 << (32+22) // D/B - Don't think we need this
            | 0x1 << (32+23) // G - Granularity, use 4KB units
            | 0x0 << (32+24) // Base 31:24
            ;

        let codeSection = dataSection 
            | 0x8 << (32+8); // Type - Execute; plus Read as we're inheriting from dataSection

        Self { 
            nullSection: 0, 
            codeSection: codeSection,
            dataSection: dataSection,
        }
    }

    pub fn load(&self) {
        let size :u16 = core::mem::size_of::<Gdt>().try_into().unwrap();
        let locator = GdtLocator {
            // "Because segment descriptors are always 8 bytes long, the GDT limit should 
            // always be one less than an integral multiple of eight (that is, 8N – 1)"
            Length: size - 1,
            BaseAddress: self.getBaseAddress(),
        };

        unsafe {
            asm!(
                "lgdt [{}]",
                in(reg) &locator,
                options(nostack, preserves_flags),
            );
        }
    }

    #[cfg(target_pointer_width = "32")]
    fn getBaseAddress(&self) -> *const Gdt {
        self
    }

    #[cfg(target_pointer_width = "64")]
    fn getBaseAddress(&self) -> u64 {
        self as *const _ as u64
    }

    // Only to be called from Real mode
    #[cfg(target_pointer_width = "32")]
    pub unsafe fn enterUnrealMode(&self) {

        self.load();

        unsafe {
            asm!(
                "push ds",
                "push ss",
                "mov  eax, cr0",
                "or   eax, 0x1",        // Set protected mode bit
                "mov  cr0, eax",        // Enter protected mode
                "mov  ax,  0x10",       // Index 2 (data segment) into the GDT
                "mov  ds,  ax",
                "mov  ss,  ax",
                "mov eax,  cr0",
                "and eax,  0xFFFFFFFE", // Clear protected mode bit
                "mov cr0,  eax",        // Restore
                "pop ss",
                "pop ds",

                out("eax") _,
            );
        }
    }

    #[cfg(target_pointer_width = "32")]
    pub unsafe fn enterProtectedMode(&self, kernel32_jump_address: usize, kernel64Address: usize, kernel64Length: usize, kernel32Address:usize, kernel32Length: usize, memoryMapAddress: usize)-> ! {
        self.load();

        vgaWriteLine!("Stage2 - K64: 0x{:X} K64L: 0x{:X} K32: 0x{:X} K32L: 0x{:X} MM: 0x{:X}", kernel64Address, kernel64Length, kernel32Address, kernel32Length, memoryMapAddress);

        unsafe {
            let kernel32Args =[kernel64Address, kernel64Length, kernel32Address, kernel32Length, memoryMapAddress];
            let argsPtr = kernel32Args.as_ptr();

            asm!(
                "mov ecx, dword ptr [eax + 0]", // Args to Kernel32
                "mov edx, dword ptr [eax + 4]",
                "push dword ptr [eax + 16]",
                "push dword ptr [eax + 12]",
                "push dword ptr [eax + 8]",
                "push 0", // BUGBUG:? Attempting to align stack
                "push 0",
                "push 0",
                "mov  eax, cr0",
                "or   eax, 0x1",    // Set protected mode bit
                "mov  cr0, eax",    // Enter protected mode
                "push 0x8",         // Index 1 (code segment) into the GDT
                "push ebx",         // Easiest way to do a long jump is a long return
                "retf",
                in("eax") argsPtr,
                in("ebx") kernel32_jump_address,
                // Not doing out for the registers we trash since we're not returning
            );
        }

        unreachable!();
    }
}

#[repr(C, packed)]
struct OurGdt {
    nullSection: u64,
    codeSection: u64,
    dataSection: u64,
    gdtSize: u16,
    selfPointer: u64,
}

// 2.4.1 Global Descriptor Table Register (GDTR)
#[repr(C, packed)]
pub struct GDTR {
    Limit: u16,
    Base: u64,
}

// Will use the highest address possible, returns the address of the GDT
pub unsafe fn Setup64BitGDT(baseAddress: u64, cantUseAbove: usize) -> usize { unsafe {

    let gdtAddress = alignDown(cantUseAbove - 1 - size_of::<OurGdt>(), GDT_ALIGNMENT);
    let baseAddress = baseAddress as usize;

    if gdtAddress < baseAddress {
        haltLoopWithMessage!("Can't put GDT @ 0x{:X}", gdtAddress);
    } else {
        loggerWriteLine!("Putting GDT @ 0x{:X}", gdtAddress);
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

    return gdtAddress;
}}

impl Gdt {
    pub fn new() -> Self {
        Gdt { 
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
        }
    }
}

impl GDTR {
    pub fn new() -> Self {
        GDTR { Limit: 0, Base: 0 }
    }

    pub unsafe fn install(&mut self, gdt: Gdt) { unsafe {
        self.Base = &gdt as *const _ as u64;
        self.Limit = size_of::<Gdt>().try_into().unwrap();

        asm!(
            "lgdt [eax]",
            in("eax") self
        );
    }}
}

#[cfg(target_pointer_width = "64")]
#[repr(C, packed)]
struct GdtrInternal {
    Length: u16,
    BaseAddress: u64,
}

#[cfg(target_pointer_width = "64")]
pub struct GdtLocator {
    pub BaseAddress: u64,
    pub Length: u16,
}

#[cfg(target_pointer_width = "64")]
pub fn GetGdtr() -> GdtLocator {
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

    GdtLocator{
        BaseAddress :base,
        Length:limit,
    }
}
