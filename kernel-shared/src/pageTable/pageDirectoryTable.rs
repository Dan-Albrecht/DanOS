use core::ptr::addr_of;

use crate::{assemblyStuff::halt::haltLoop, haltLoopWithMessage, memoryTypes::PhysicalAddress};

use super::{enums::*, pageTable::PageTable};

#[repr(C, packed)]
pub struct PageDirectoryTable {
    // PDE
    // (AMD64 Volume2) Figure 5-22. 4-Kbyte PDE—Long Mode
    Entries: [u64; 512],
}

impl PageDirectoryTable {
    pub fn setEntry(
        &mut self,
        index: usize,
        pt: &PhysicalAddress<PageTable>,
        executable: Execute,
        present: Present,
        writable: Writable,
        cachable: Cachable,
        us: UserSupervisor,
        wt: WriteThrough,
    ) {
        let entry = Self::calculateEntry(pt, executable, present, writable, cachable, us, wt);

        self.Entries[index] = entry;
    }

    fn calculateEntry(
        entry: &PhysicalAddress<PageTable>,
        executable: Execute,
        present: Present,
        writable: Writable,
        cachable: Cachable,
        us: UserSupervisor,
        wt: WriteThrough,
    ) -> u64 {
        let address = entry.address as u64;
        let maskedAddress = address & 0xFFFFFFFFFF000;

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

        // 7 Must Be Zero

        // 8 is ignored

        // 9-11 Available, but we don't use them

        // 12-51 Pointer to next structure. We've checked this with mask above.

        // 52-62 Available, but we don't use them

        if executable == Execute::No {
            // (NX)
            result |= 1 << 63;
        }

        return result;
    }

    pub fn getAddressForEntry(&self, index: usize) -> PhysicalAddress<PageTable> {
        let mut entry = self.Entries[index];
        entry = entry & 0xF_FFFF_FFFF_F000;

        PhysicalAddress::<PageTable>::new(entry as usize)
    }
    
    pub fn getNumberOfEntries(&self) -> usize {
        let entries = addr_of!(self.Entries);

        unsafe { (*entries).len() }
    }
}
