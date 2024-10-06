use crate::{alignment::PageAligned, assemblyStuff::halt::haltLoop, haltLoopWithMessage};

use super::{enums::*, pageDirectoryPointerTable::PageDirectoryPointerTable};
use core::{array::from_fn, fmt::Write};

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
        pdpt: *const PageDirectoryPointerTable,
        executable: Execute,
        present: Present,
        writable: Writable,
        cachable: Cachable,
        us: UserSupervisor,
        wt: WriteThrough,
    ) {
        let entry = Self::calculateEntry(pdpt, executable, present, writable, cachable, us, wt);

        self.Entries[index] = entry;
    }

    fn calculateEntry(
        entry: *const PageDirectoryPointerTable,
        executable: Execute,
        present: Present,
        writable: Writable,
        cachable: Cachable,
        us: UserSupervisor,
        wt: WriteThrough,
    ) -> u64 {
        let address = entry as u64;
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

        // 7-8 Must Be Zero

        // 9-11 Available, but we don't use them

        // 12-51 Pointer to next structure. We've checked this with mask above.

        // 52-62 Available, but we don't use them

        if executable == Execute::No {
            // (NX)
            result |= 1 << 63;
        }

        return result;
    }

    pub fn getAddressForEntry(&self, index: usize) -> *mut PageDirectoryPointerTable {
        let mut entry = self.Entries[index];
        entry = entry & 0xF_FFFF_FFFF_F000;

        return entry as *mut PageDirectoryPointerTable;
    }
}
