use core::fmt::Write;
use core::{arch::asm, mem::size_of};


use crate::assemblyStuff::halt::haltLoop;
use crate::magicConstants::{ENTRIES_PER_PAGE_TABLE, FIRST_PD, FIRST_PDPT, FIRST_PML4, FIRST_PT};
use crate::{
    memoryHelpers::{alignUp, haltOnMisaligned, zeroMemory2},
    vgaWriteLine,
};

use super::{
    pageDirectoryPointerTable::PageDirectoryPointerTable, pageDirectoryTable::PageDirectoryTable,
    pageMapLevel4Table::PageMapLevel4Table, pageTable::PageTable, physicalPage::PhysicalPage,
};

// This is the top of the hiearchy. Would have called this ThePageTable,
// but we already have a PageTable type much lower in the hierarchy.
pub struct PageBook {
    // This could be a PML5 if we ever wanted to support the extra bits of addressing
    // Assuming CR4.PCIDE=0
    Entry: u64,
}

impl PageBook {

    fn new() -> PageBook{
        PageBook{
            Entry:0,
        }
    }

    // This will create and initalize
    pub fn fromScratch() -> PageBook {
        unsafe {

            let pt = FIRST_PT as * mut PageTable;
            // BUGBUG: Make these compile time, we have consts
            // Also the spacing of subsequent addresses should be validated to make sure the size of the struct doesn't overlap
            haltOnMisaligned("PT", pt as usize, 0x1000);
            vgaWriteLine!("PT @ 0x{:X}", pt as usize);
            zeroMemory2(pt);

            let pdt = FIRST_PD as * mut PageDirectoryTable;
            haltOnMisaligned("PDT", pdt as usize, 0x1000);
            vgaWriteLine!("PDT @ 0x{:X}", pdt as usize);
            zeroMemory2(pdt);

            let pdpt = FIRST_PDPT as * mut PageDirectoryPointerTable;
            haltOnMisaligned("PDPT", pdpt as usize, 0x1000);
            vgaWriteLine!("PDPT @ 0x{:X}", pdpt as usize);
            zeroMemory2(pdpt);

            let pml4 = FIRST_PML4 as *mut PageMapLevel4Table;
            haltOnMisaligned("PML4", pml4 as usize, 0x1000);
            vgaWriteLine!("PML4 @ 0x{:X}", pml4 as usize);
            zeroMemory2(pml4);

            // BUGUBG: This thing is given way to much space
            let mut pb = PageBook::new();

            for index in 0..ENTRIES_PER_PAGE_TABLE {
                let page = index * size_of::<PhysicalPage>();
                // BUGUBG: We're setting these uncachable for now just to be extra safe, but shouldn't be needed anymore...
                // BUGBUG: This method now allows bulk setting of pages...
                (*pt).setEntry(index, 1, page, true, true, false);
            }

            (*pdt).setEntry(0, pt, true, true, false);
            (*pdpt).setEntry(0, pdt, true, true, false);
            (*pml4).setEntry(0, pdpt, true, true, false);
            pb.setEntry(pml4, false, false);

            return pb;
        }
    }

    // This will just blindly assume you've already created this
    #[cfg(target_pointer_width = "64")]
    pub unsafe fn fromExisting64() -> PageBook {
        let cr3: u64;

        asm!(
            "mov rax, cr3",
            out("rax") cr3,
        );

        return PageBook { Entry: cr3 as u64 };
    }

    pub fn setEntry(&mut self, pml4: *const PageMapLevel4Table, pcd: bool, pwt: bool) {
        let mut address = pml4 as usize;

        haltOnMisaligned("PML4", address, 0x1000);

        // Page-Level Cache Disable (PCD) Bit 4
        if pcd {
            address |= 0b1_0000;
        }

        // Page-Level Writethrough (PWT) Bit. Bit 3
        if pwt {
            address |= 0b1000;
        }

        self.Entry = address as u64;
    }

    pub fn getEntry(&self) -> *const PageMapLevel4Table {
        (self.Entry & (!0xFFF)) as *const PageMapLevel4Table
    }

    pub fn getCR3Value(&self) -> u64 {
        self.Entry
    }

    pub unsafe fn initNewPageTable(
        pt: *mut PageTable,
        startAddress: usize,
        pageIndex: usize,
        numberOfPages: usize,
    ) {
        zeroMemory2(pt);
        vgaWriteLine!(
            "New PT @ 0x{:X} settings index {} to 0x{:X}",
            pt as usize,
            pageIndex,
            startAddress
        );

        for index in 0..numberOfPages {
            let page = startAddress + (index * size_of::<PhysicalPage>());
            
            // BUGBUG: Expose flags to upstream
            // BUGBUG: This method now supports bulk setting of pages
            (*pt).setEntry(index + pageIndex, 1, page, true, true, false);
        }
    }
}
