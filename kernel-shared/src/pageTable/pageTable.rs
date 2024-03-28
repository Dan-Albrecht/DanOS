#[repr(C, packed)]
pub struct PageTable {
    // PTE
    // The actual physical page address with some extra metadata bits or'd in
    Entries: [u64; 512],
}

impl PageTable {
    pub fn setEntry(
        &mut self,
        index: usize,
        address: u64,
        present: bool,
        writable: bool,
        cachable: bool,
    ) {
        let mut address = address;

        assert!(address & 0xFFF == 0, "Entry isn't properly aligned");
        assert!(
            address & 0xFFF0_0000_0000_0000 != 0,
            "This is more memory that the processor supports..."
        );

        if present {
            address |= 1;
        }

        if writable {
            address |= 1 << 1;
        }

        if !cachable {
            address |= 1 << 4;
        }

        self.Entries[index] = address;
    }

    pub fn getAddressForEntry(&self, index: usize) -> u64 {
        let mut entry = self.Entries[index];
        entry = entry & 0xF_FFFF_FFFF_F000;

        return entry;
    }
}
