use core::arch::asm;

pub unsafe fn enablePaging() {

    // Previously tried to do this in Rust-Assembly hybrid and failed miserably
    // try pure assembly for now. Directly from:
    // https://wiki.osdev.org/Setting_Up_Long_Mode
    // just to prove we can shoehorn this in.
    // BUGBUG: Revisit after we write the real Rust code to modify the page table
    // in 64-bit mode.
    asm!(
        "mov edi, 0x1000",
        "mov cr3, edi",
        "xor eax, eax",
        "mov ecx, 4096",
        "rep stosd",
        "mov edi, cr3",
        //
        "mov DWORD PTR [edi], 0x2003",
        "add edi, 0x1000",
        "mov DWORD PTR [edi], 0x3003",
        "add edi, 0x1000",
        "mov DWORD PTR [edi], 0x4003",
        "add edi, 0x1000",
        //
        "mov ebx, 0x00000003",
        "mov ecx, 512",
        "2:",
        "mov DWORD PTR [edi], ebx",
        "add ebx, 0x1000",
        "add edi, 8 ",
        "loop 2b",
        //
        "mov eax, cr4",
        "or eax, 1 << 5",
        "mov cr4, eax",
        //
        "mov ecx, 0xC0000080",
        "rdmsr",
        "or eax, 1 << 8",
        "wrmsr",
        //
        "mov eax, cr0",
        "or eax, 1 << 31",
        "mov cr0, eax",
        // Clobbers:
        out("edi") _,
        out("eax") _,
        out("ebx") _,
        out("ecx") _,
    );
}
