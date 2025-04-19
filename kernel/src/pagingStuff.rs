use core::arch::asm;

use kernel_shared::{
    loggerWriteLine, memory::map::MemoryMap, pageTable::pageBook::{CreationResult, PageBook}
};


pub fn enablePaging(memoryMap: &MemoryMap) -> CreationResult {
    unsafe {
        loggerWriteLine!("Enabling PAE");
        enablePae();
        loggerWriteLine!("Setting page data");
        let pageTableData = setPageData(memoryMap);
        loggerWriteLine!("Enabling long mode");
        enableLongMode();
        loggerWriteLine!("Enabling paging");
        reallyEnablePaging();

        return pageTableData;
    }
}

// 5.1.3 Physical-Address Extensions (PAE) Bit
unsafe fn enablePae() { unsafe {
    asm!(
        "mov eax, cr4",
        // 5.1.3 Physical-Address Extensions (PAE) Bit
        "bts eax, 5",
        "mov cr4, eax",
        // Clobbers:
        out("eax") _,
    );
}}

unsafe fn enableLongMode() { unsafe {
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
}}

unsafe fn setPageData(memoryMap: &MemoryMap) -> CreationResult { unsafe {
    loggerWriteLine!("Creating book");
    let result = PageBook::fromScratch(memoryMap);
    let cr3 = result.Book.getCR3Value();

    loggerWriteLine!("Restier cr3 to 0x{:X}", cr3);
    asm!(
        "mov cr3, eax",
        in("eax") cr3 as u32,
    );

    return result;
}}

unsafe fn reallyEnablePaging() { unsafe {
    asm!(
        "mov eax, cr0",
        // 5.1.2 Page-Translation Enable (PG) Bit
        "bts eax, 31",
        "mov cr0, eax",
        // Clobbers:
        out("eax") _,
    );
}}
