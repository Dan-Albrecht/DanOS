use core::arch::asm;
use core::fmt::Write;

use kernel_shared::{assemblyStuff::halt::haltLoop, pageTable::pageBook::PageBook, vgaWriteLine};

pub fn enablePaging() {
    unsafe {
        vgaWriteLine!("Enabling PAE");
        enablePae();
        vgaWriteLine!("Setting page data");
        setPageData();
        vgaWriteLine!("Enabling long mode");
        enableLongMode();
        vgaWriteLine!("Enabling paging");
        reallyEnablePaging();
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

unsafe fn setPageData() {
    vgaWriteLine!("Getting book");
    let book = PageBook::fromScratch();
    let cr3 = (*book).getCR3Value();
    if cr3 > u32::MAX as u64 {
        vgaWriteLine!("Page table structs in 64-bit space, but we're still in 32");
        haltLoop();
    }
    vgaWriteLine!("Registering book at 0x{:X}", cr3);
    asm!(
        "mov cr3, eax",
        in("eax") cr3 as u32,
    );
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
