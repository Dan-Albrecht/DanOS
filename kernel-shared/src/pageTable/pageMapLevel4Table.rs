use crate::{alignment::PageAligned, assemblyStuff::halt::haltLoop, haltLoopWithMessage, memoryTypes::PhysicalAddress, vgaWriteLine};

use super::{enums::*, pageDirectoryPointerTable::PageDirectoryPointerTable};
use core::{array::from_fn, ptr::addr_of};

use crate::memoryTypes::SomeSortOfIndex;

#[repr(C, packed)]
pub struct PageMapLevel4Table {
    // PML4E
    // (AMD64 Volume2) Figure 5-20. 4-Kbyte PML4E—Long Mode
    Entries: [u64; 512],
}

impl PageMapLevel4Table {
    pub fn new() -> PageAligned<Self> {
        PageAligned {
            field: PageMapLevel4Table {
                Entries: from_fn(|_| 0),
            },
        }
    }

    pub fn setEntry(
        &mut self,
        index: usize,
        pdpt: &PhysicalAddress<PageDirectoryPointerTable>,
        executable: Execute,
        present: Present,
        writable: Writable,
        cachable: Cachable,
        us: UserSupervisor,
        wt: WriteThrough,
        xxx: SomeSortOfIndex,
    ) {
        vgaWriteLine!("setEntry XXX is {}", xxx.value);
        let entry = Self::calculateEntry(pdpt, executable, present, writable, cachable, us, wt, xxx);

        self.Entries[index] = entry;
    }

    fn calculateEntry(
        entry: &PhysicalAddress<PageDirectoryPointerTable>,
        executable: Execute,
        present: Present,
        writable: Writable,
        cachable: Cachable,
        us: UserSupervisor,
        wt: WriteThrough,
        xxx: SomeSortOfIndex,
    ) -> u64 {
        let address = entry.address as u64;
        let maskedAddress = address & 0xF_FFFF_FFFF_F000;

        if address != maskedAddress {
            // Either misaligned or setting bits they shouldn't be
            haltLoopWithMessage!("Address 0x{:X} contains masked bits", address);
        }

        let mut result = maskedAddress;

        if present == Present::Yes {
            // (P)
            result |= 1 << 0;
        }

        if writable == Writable::Yes {
            // (R/W)
            result |= 1 << 1;
        }

        if us == UserSupervisor::Supervisor {
            // (U/S)
            result |= 1 << 2;
        }

        if wt == WriteThrough::WriteTrough {
            // (PWT)
            result |= 1 << 3;
        }

        if cachable == Cachable::No {
            // (PCD)
            result |= 1 << 4;
        }

        // Accessed = 5

        // 6 is ignored

        // 7-8 Must Be Zero

        // 9-11 Available, but we don't use them

        // 12-51 Pointer to next structure. We've checked this with mask above.

        // 52-62 Available; using 8 of them as an index into a Physical to Virtual lookup array
        let temp = (xxx.value as u64) << 52;
        vgaWriteLine!("calculateEntry XXX is {}", temp);
        result |= temp;

        if executable == Execute::No {
            // (NX)
            result |= 1 << 63;
        }

        return result;
    }

    #[cfg(target_pointer_width = "64")]
    pub fn getSomeSortOfIndex(&self, index: usize) -> SomeSortOfIndex {        
        let entry = self.Entries[index];
        let value = (entry >> 52) & 0xFF;
        vgaWriteLine!("getSomeSortOfIndex XXX is {}", value);

        SomeSortOfIndex{
            value : value as u8
        }
    }

    pub fn getAddressForEntry(&self, index: usize) -> PhysicalAddress<PageDirectoryPointerTable> {
        let mut entry = self.Entries[index];
        entry = entry & 0xF_FFFF_FFFF_F000;

        PhysicalAddress::<PageDirectoryPointerTable>::new(entry as usize)
    }

    pub fn getNumberOfEntries(&self) -> usize {
        let entries = addr_of!(self.Entries);

        unsafe { (*entries).len() }
    }
}
