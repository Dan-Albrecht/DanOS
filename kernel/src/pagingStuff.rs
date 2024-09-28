use core::arch::asm;
use core::fmt::Write;

use kernel_shared::{
    memoryMap::MemoryMap, pageTable::pageBook::PageBook,
    vgaWriteLine,
};


pub fn enablePaging(memoryMap: &MemoryMap) -> usize {
    unsafe {
        vgaWriteLine!("Enabling PAE");
        enablePae();
        vgaWriteLine!("Setting page data");
        let cantUseAbove = setPageData(memoryMap);
        vgaWriteLine!("Enabling long mode");
        enableLongMode();
        vgaWriteLine!("Enabling paging");
        reallyEnablePaging();

        return cantUseAbove;
    }
}

// 5.1.3 Physical-Address Extensions (PAE) Bit
unsafe fn enablePae() {
    asm!(
        "mov eax, cr4",
        // 5.1.3 Physical-Address Extensions (PAE) Bit
        "bts eax, 5",
        "mov cr4, eax",
        // Clobbers:
        out("eax") _,
    );
}

unsafe fn enableLongMode() {
    asm!(
        // 3.1.7 Extended Feature Enable Register (EFER)
        "mov ecx, 0xC0000080",
        "rdmsr",
        // Long Mode Enable (LME) Bit
        "bts eax, 8",
        "wrmsr",
        // Clobbers:
        out("eax") _,
        out("ecx") _,
    );
}

unsafe fn setPageData(memoryMap: &MemoryMap) -> usize {
    vgaWriteLine!("Getting book");
    let result = PageBook::fromScratch(memoryMap);
    let cr3 = result.Book.getCR3Value();

    vgaWriteLine!("Restier cr3 to 0x{:X}", cr3);
    asm!(
        "mov cr3, eax",
        in("eax") cr3 as u32,
    );

    return result.LowestPhysicalAddressUsed;
}

unsafe fn reallyEnablePaging() {
    asm!(
        "mov eax, cr0",
        // 5.1.2 Page-Translation Enable (PG) Bit
        "bts eax, 31",
        "mov cr0, eax",
        // Clobbers:
        out("eax") _,
    );
}
