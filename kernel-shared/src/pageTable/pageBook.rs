use core::fmt::Write;
use core::mem::size_of;

use crate::memoryHelpers::alignDown;
use crate::memoryMap::{MemoryMap, MemoryMapEntryType};
use crate::pageTable::pageTable::ENTRIES_PER_PAGE_TABLE;
use crate::{
    haltLoopWithMessage,
    memoryHelpers::{haltOnMisaligned, zeroMemory2},
    vgaWriteLine,
};

use crate::assemblyStuff::halt::haltLoop;

use super::{
    pageDirectoryPointerTable::PageDirectoryPointerTable, pageDirectoryTable::PageDirectoryTable,
    pageMapLevel4Table::PageMapLevel4Table, pageTable::PageTable, physicalPage::PhysicalPage,
};

const PAGE_STRUCTURE_ALIGNMENT: usize = 0x1000;

// This is the top of the hiearchy. Would have called this ThePageTable,
// but we already have a PageTable type much lower in the hierarchy.
pub struct PageBook {
    // This could be a PML5 if we ever wanted to support the extra bits of addressing
    // Assuming CR4.PCIDE=0
    Entry: u64,
}

pub struct CreationResult {
    pub Book: PageBook,
    pub LowestPhysicalAddressUsed: usize,
}

impl PageBook {
    fn new() -> PageBook {
        PageBook { Entry: 0 }
    }

    // This will create and initalize, uses memory from the first memory map entry
    pub fn fromScratch(memoryMap: &MemoryMap) -> CreationResult {
        unsafe {
            // We're being lazy, but safe. Want the first entry to be usable memory and big enough so we can at least allocate the page structure in it.
            let entry = memoryMap.Entries[0];
            if entry.GetType() != MemoryMapEntryType::AddressRangeMemory {
                haltLoopWithMessage!("Add better PageTable setup code");
            }

            let maxAddress = entry.BaseAddr + entry.Length - 1;

            if maxAddress & 0xFFFF_FFFF_0000_0000 != 0 {
                haltLoopWithMessage!("Address extends beyond 32-bit space and I want easy casting");
            }

            let maxAddress = maxAddress as usize;

            // +1 as we're currently pointing at the last byte instead of one beyond like the rest of these will be
            let pt = alignDown(
                maxAddress - size_of::<PageTable>() + 1,
                PAGE_STRUCTURE_ALIGNMENT,
            );
            let pt = pt as *mut PageTable;
            haltOnMisaligned("PT", pt as usize, PAGE_STRUCTURE_ALIGNMENT);
            vgaWriteLine!("PT @ 0x{:X}", pt as usize);
            zeroMemory2(pt);

            let pdt = alignDown(
                pt as usize - size_of::<PageDirectoryTable>(),
                PAGE_STRUCTURE_ALIGNMENT,
            );
            let pdt = pdt as *mut PageDirectoryTable;
            vgaWriteLine!("PDT @ 0x{:X}", pdt as usize);
            zeroMemory2(pdt);

            let pdpt = alignDown(
                pdt as usize - size_of::<PageDirectoryPointerTable>(),
                PAGE_STRUCTURE_ALIGNMENT,
            );
            let pdpt = pdpt as *mut PageDirectoryPointerTable;
            vgaWriteLine!("PDPT @ 0x{:X}", pdpt as usize);
            zeroMemory2(pdpt);

            let pml4 = alignDown(
                pdpt as usize - size_of::<PageMapLevel4Table>(),
                PAGE_STRUCTURE_ALIGNMENT,
            );
            let pml4 = pml4 as *mut PageMapLevel4Table;
            vgaWriteLine!("PML4 @ 0x{:X}", pml4 as usize);
            zeroMemory2(pml4);

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

            return CreationResult {
                Book: pb,
                LowestPhysicalAddressUsed: pml4 as usize,
            };
        }
    }

    #[cfg(target_pointer_width = "64")]
    pub fn fromExisting() -> PageBook {
        unsafe {
            // This will just blindly assume you've already created this
            // Given we've marked the funciton 64-bit only, seems reasonably safe
            // to assume we have paging setup already.
            let cr3: u64;

            core::arch::asm!(
                "mov rax, cr3",
                out("rax") cr3,
            );

            return PageBook { Entry: cr3 as u64 };
        }
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
