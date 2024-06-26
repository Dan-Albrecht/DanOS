use core::fmt::Write;
use core::mem::size_of;

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
    // This will create and initalize
    pub fn fromScratch() -> *const PageBook {
        unsafe {
            //let pt = size_of::<PhysicalPage>() * 512;
            // BUGBUG: We want all the paging structure within paged memory, I think
            let pt = 0x10_0000;
            let pt = alignUp(pt, 0x1000) as *mut PageTable;
            zeroMemory2(pt);
            vgaWriteLine!("PageTable @ 0x{:X}", pt as usize);

            for index in 0..512 {
                let page = (index * size_of::<PhysicalPage>()) as *mut PhysicalPage;
                // BUGBUG: Cannot zero all these because this contains the code we're actually running at right now
                //zeroMemory2(page);
                // BUGBUG: Setting this uncachable for now as we're going to map the hard drive in this space
                // Need to get it, its own page
                (*pt).setEntry(index, page, true, true, false);
            }

            let pdt = pt as usize + size_of::<PageTable>();
            let pdt = alignUp(pdt, 0x1000) as *mut PageDirectoryTable;
            zeroMemory2(pdt);
            vgaWriteLine!("PDT @ 0x{:X}", pdt as usize);
            (*pdt).setEntry(0, pt, true, true, false);

            let pdpt = pdt as usize + size_of::<PageDirectoryTable>();
            let pdpt = alignUp(pdpt, 0x1000) as *mut PageDirectoryPointerTable;
            zeroMemory2(pdpt);
            vgaWriteLine!("PDPT @ 0x{:X}", pdpt as usize);
            (*pdpt).setEntry(0, pdt, true, true, false);

            let pml4 = pdpt as usize + size_of::<PageDirectoryPointerTable>();
            let pml4 = alignUp(pml4, 0x1000) as *mut PageMapLevel4Table;
            zeroMemory2(pml4);
            vgaWriteLine!("PML4 @ 0x{:X}", pml4 as usize);
            (*pml4).setEntry(0, pdpt, true, true, false);

            let pageBook = pml4 as usize + size_of::<PageMapLevel4Table>();
            let pageBook = alignUp(pageBook, 0x1000) as *mut PageBook;
            zeroMemory2(pageBook);
            vgaWriteLine!("PageBook @ 0x{:X}", pageBook as usize);
            (*pageBook).setEntry(pml4, false, false);

            return pageBook;
        }
    }

    // This will just blindly assume you've already created this
    pub fn fromExisting() -> *const PageBook {
        // Probalby just pull from teh control registers?
        todo!()
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
}
