use core::mem::size_of;
use core::u8;

use crate::loggerWriteLine;
use crate::magicConstants::SIZE_OF_PAGE;
use crate::memory::map::MemoryMap;
use crate::memory::mapEntry::MemoryMapEntryType;
use crate::memoryHelpers::alignDown;
use crate::memoryTypes::{PhysicalAddress, SomeSortOfIndex, VirtualAddress};
use crate::pageTable::enums::*;
use crate::pageTable::pageTable::ENTRIES_PER_PAGE_TABLE;
use crate::{
    haltLoopWithMessage,
    memoryHelpers::{haltOnMisaligned, zeroMemory2},
};

use crate::assemblyStuff::halt::haltLoop;

use super::{
    pageDirectoryPointerTable::PageDirectoryPointerTable, pageDirectoryTable::PageDirectoryTable,
    pageMapLevel4Table::PageMapLevel4Table, pageTable::PageTable, physicalPage::PhysicalPage,
};

// This is the top of the hiearchy. Would have called this ThePageTable,
// but we already have a PageTable type much lower in the hierarchy.
pub struct PageBook {
    // This could be a PML5 if we ever wanted to support the extra bits of addressing
    // Assuming CR4.PCIDE=0
    // This is a physicall address with some potential extra control bits or'd in.
    PhysicalAddressEntry: u64,
    // No metadata in this, this is the actual address
    VirtualAddress: VirtualAddress<PageMapLevel4Table>,
}

pub struct CreationResult {
    pub Book: PageBook,
    pub PageStructuresStart: usize,
    pub PageStructuresEnd: usize,
}

impl PageBook {
    pub fn new(
        pcd: bool,
        pwt: bool,
        physicalAddress: PhysicalAddress<PageMapLevel4Table>,
        virtualAddress: VirtualAddress<PageMapLevel4Table>,
    ) -> PageBook {
        let mut entry = physicalAddress.address;
        haltOnMisaligned("PML4", entry, SIZE_OF_PAGE);

        // Page-Level Cache Disable (PCD) Bit 4
        if pcd {
            entry |= 1 << 4;
        }

        // Page-Level Writethrough (PWT) Bit. Bit 3
        if pwt {
            entry |= 1 << 3;
        }

        PageBook {
            PhysicalAddressEntry: entry.try_into().unwrap(),
            VirtualAddress: virtualAddress,
        }
    }

    fn createPageTable(address: usize, multiplier: usize) -> PhysicalAddress<PageTable> {
        let pt = alignDown(address, SIZE_OF_PAGE);
        haltOnMisaligned("PT", pt as usize, SIZE_OF_PAGE);
        loggerWriteLine!("PT @ 0x{:X}", pt as usize);
        let pt = PhysicalAddress::<PageTable>::new(pt);
        unsafe {
            zeroMemory2(pt.unsafePtr());
        }

        for index in 0..ENTRIES_PER_PAGE_TABLE {
            let page = (index + (multiplier * ENTRIES_PER_PAGE_TABLE)) * size_of::<PhysicalPage>();
            let page = PhysicalAddress::<PhysicalPage>::new(page);
            // BUGUBG: We're setting these uncachable for now just to be extra safe, but shouldn't be needed anymore...
            unsafe {
                (*pt.unsafePtr()).setEntry(
                    index,
                    &page,
                    Execute::Yes,
                    Present::Yes,
                    Writable::Yes,
                    Cachable::No,
                    UserSupervisor::Supervisor,
                    WriteThrough::WriteTrough,
                );
            }
        }

        return pt;
    }

    // This will create and initalize, uses memory from the first memory map entry
    // BUGBUG: Need to at least double the identity mapping we're doing; can't relocate kernl64 otherwise
    pub fn fromScratch(memoryMap: &MemoryMap) -> CreationResult {
        unsafe {
            // We're being lazy, but safe. Want the first entry to be usable memory and big enough so we can at least allocate the page structure in it.
            let entry = memoryMap.Entries[0];
            if entry.getType() != MemoryMapEntryType::AddressRangeMemory {
                haltLoopWithMessage!("Add better PageTable setup code");
            }

            let maxAddress = entry.BaseAddress + entry.Length - 1;

            if maxAddress & 0xFFFF_FFFF_0000_0000 != 0 {
                haltLoopWithMessage!("Address extends beyond 32-bit space and I want easy casting");
            }

            let maxAddress = maxAddress as usize;

            // +1 as we're currently pointing at the last byte instead of one beyond like the rest of these will be
            let pt = Self::createPageTable(maxAddress - size_of::<PageTable>() + 1, 0);
            let pt2 = Self::createPageTable(pt.address - size_of::<PageTable>(), 1);
            let pt3 = Self::createPageTable(pt2.address - size_of::<PageTable>(), 2);
            let pt4 = Self::createPageTable(pt3.address - size_of::<PageTable>(), 3);
            let pt5 = Self::createPageTable(pt4.address - size_of::<PageTable>(), 4);
            let pt6 = Self::createPageTable(pt5.address - size_of::<PageTable>(), 5);
            let lastPt = pt6;

            let pdt = alignDown(lastPt.address - size_of::<PageDirectoryTable>(), SIZE_OF_PAGE);
            loggerWriteLine!("PDT @ 0x{:X}", pdt as usize);
            // BUGUBG: Zero the virtual
            //zeroMemory2(pdt);
            let pdt = PhysicalAddress::<PageDirectoryTable>::new(pdt);

            let pdpt = alignDown(
                pdt.address - size_of::<PageDirectoryPointerTable>(),
                SIZE_OF_PAGE,
            );

            loggerWriteLine!("PDPT @ 0x{:X}", pdpt);
            let pdpt = PhysicalAddress::<PageDirectoryPointerTable>::new(pdpt);

            // BUGBUG: How was this working before if this was a physical address that may not have been identity mapped...
            // zeroMemory2(pdpt);

            let pml4 = alignDown(pdpt.address - size_of::<PageMapLevel4Table>(), SIZE_OF_PAGE);
            let pml4 = pml4 as *mut PageMapLevel4Table;
            loggerWriteLine!("PML4 @ 0x{:X}", pml4 as usize);
            zeroMemory2(pml4);

            let startAddress = (*pt.unsafePtr()).getAddressForEntry(0);
            let endAddress =
                (*lastPt.unsafePtr()).getAddressForEntry((*lastPt.unsafePtr()).getNumberOfEntries() - 1);
            loggerWriteLine!(
                "Identity mapped: 0x{:X}..0x{:X}",
                startAddress.address,
                endAddress.address + size_of::<PhysicalPage>(),
            );

            (*pdt.unsafePtr()).setEntry(
                0,
                &pt,
                Execute::Yes,
                Present::Yes,
                Writable::Yes,
                Cachable::No,
                UserSupervisor::Supervisor,
                WriteThrough::WriteTrough,
            );
            (*pdt.unsafePtr()).setEntry(
                1,
                &pt2,
                Execute::Yes,
                Present::Yes,
                Writable::Yes,
                Cachable::No,
                UserSupervisor::Supervisor,
                WriteThrough::WriteTrough,
            );
            (*pdt.unsafePtr()).setEntry(
                2,
                &pt3,
                Execute::Yes,
                Present::Yes,
                Writable::Yes,
                Cachable::No,
                UserSupervisor::Supervisor,
                WriteThrough::WriteTrough,
            );
            (*pdt.unsafePtr()).setEntry(
                3,
                &pt4,
                Execute::Yes,
                Present::Yes,
                Writable::Yes,
                Cachable::No,
                UserSupervisor::Supervisor,
                WriteThrough::WriteTrough,
            );
            (*pdt.unsafePtr()).setEntry(
                4,
                &pt5,
                Execute::Yes,
                Present::Yes,
                Writable::Yes,
                Cachable::No,
                UserSupervisor::Supervisor,
                WriteThrough::WriteTrough,
            );
            (*pdt.unsafePtr()).setEntry(
                5,
                &lastPt,
                Execute::Yes,
                Present::Yes,
                Writable::Yes,
                Cachable::No,
                UserSupervisor::Supervisor,
                WriteThrough::WriteTrough,
            );

            (*pdpt.unsafePtr()).setEntry(
                0,
                &pdt,
                Execute::Yes,
                Present::Yes,
                Writable::Yes,
                Cachable::No,
                UserSupervisor::Supervisor,
                WriteThrough::WriteTrough,
            );
            (*pml4).setEntry(
                0,
                &pdpt,
                Execute::Yes,
                Present::Yes,
                Writable::Yes,
                Cachable::No,
                UserSupervisor::Supervisor,
                WriteThrough::WriteTrough,
                SomeSortOfIndex { value: u8::MAX },
            );

            // BUGBUG: This is assuming we're currently identity mapped
            let pb = PageBook::new(
                false,
                false,
                PhysicalAddress::<PageMapLevel4Table>::new(pml4 as usize),
                VirtualAddress::<PageMapLevel4Table>::new(pml4 as usize),
            );

            return CreationResult {
                Book: pb,
                PageStructuresStart: pml4 as usize,
                PageStructuresEnd: maxAddress,
            };
        }
    }

    #[cfg(target_pointer_width = "64")]
    pub fn fromExistingIdentityMapped() -> PageBook {
        unsafe {
            // This will just blindly assume you've already created this
            // Given we've marked the funciton 64-bit only, seems reasonably safe
            // to assume we have paging setup already.
            let cr3: u64;

            core::arch::asm!(
                "mov rax, cr3",
                out("rax") cr3,
            );

            let mut result = PageBook {
                PhysicalAddressEntry: cr3,
                VirtualAddress: VirtualAddress::<PageMapLevel4Table>::new(0),
            };

            // We're just doing this as a roundabout way to mask out the extra data so that logic is in a single spot
            let virtualAddress = result.getPhysical().address;
            result.VirtualAddress = VirtualAddress::<PageMapLevel4Table>::new(virtualAddress);

            return result;
        }
    }

    pub fn getPhysical(&self) -> PhysicalAddress<PageMapLevel4Table> {
        let maskedAddress = self.PhysicalAddressEntry & (!0xFFF);
        PhysicalAddress::new(maskedAddress.try_into().unwrap())
    }

    pub fn getVirtual(&self) -> VirtualAddress<PageMapLevel4Table> {
        VirtualAddress::<PageMapLevel4Table>::new(self.VirtualAddress.address)
    }

    pub fn getCR3Value(&self) -> u64 {
        self.PhysicalAddressEntry
    }
}
