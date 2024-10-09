use crate::{
    assemblyStuff::halt::haltLoop, haltLoopWithMessage,
};

use super::{enums::*, physicalPage::PhysicalPage};
use core::fmt::Write;

pub(crate) const ENTRIES_PER_PAGE_TABLE: usize = 512;

#[repr(C, packed)]
pub struct PageTable {
    // PTE
    // (AMD64 Volume2) Figure 5-23. 4-Kbyte PTE—Long Mode
    Entries: [u64; ENTRIES_PER_PAGE_TABLE],
}

impl PageTable {
    pub fn setEntry(
        &mut self,
        index: usize,
        physicalAddress: u64,
        executable: Execute,
        present: Present,
        writable: Writable,
        cachable: Cachable,
        us: UserSupervisor,
        wt: WriteThrough,
    ) {
        let entry = Self::calculateEntry(
            physicalAddress,
            executable,
            present,
            writable,
            cachable,
            us,
            wt,
        );
        self.Entries[index] = entry;
    }

    fn calculateEntry(
        address: u64,
        executable: Execute,
        present: Present,
        writable: Writable,
        cachable: Cachable,
        us: UserSupervisor,
        wt: WriteThrough,
    ) -> u64 {
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

        // Dirty = 6

        // Page-Attribute Table (PAT) = 7
        // We're not using it

        // Global Page (G) = 8
        // We're not using it

        // 9-11 Available, but we don't use them

        // 12-51 Pointer to next structure. We've checked this with mask above.

        // 52-58 Available, but we don't use them

        // 59-69 Available as we're not using Memory Protection Key(PKE), but we're not using these bits

        if executable == Execute::No {
            // (NX)
            result |= 1 << 63;
        }

        return result;
    }

    pub fn getAddressForEntry(&self, index: usize) -> *const PhysicalPage {
        let mut entry = self.Entries[index];
        entry = entry & 0xF_FFFF_FFFF_F000;

        return entry as *const PhysicalPage;
    }
}
