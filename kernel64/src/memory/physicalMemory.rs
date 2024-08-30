use core::{array::from_fn, fmt::Write, ptr::addr_of_mut};
use kernel_shared::{
    assemblyStuff::halt::haltLoop, haltLoopWithMessage,
    magicConstants::ADDRESS_OF_MEMORY_MANAGER_BEFORE_HEAP, vgaWriteLine,
};

use super::memoryMap::{MemoryMap, MemoryMapEntryType};

pub struct PhysicalMemoryManager {
    MemoryMap: MemoryMap,
    Blobs: [MemoryBlob; 100],
}

struct MemoryBlob {
    Address: usize,
    Length: usize,
}

impl Default for MemoryBlob {
    fn default() -> Self {
        MemoryBlob {
            Address: 0,
            Length: 0,
        }
    }
}

impl PhysicalMemoryManager {
    pub fn Init(memoryMap: MemoryMap) -> *mut PhysicalMemoryManager {
        unsafe {
            let result = ADDRESS_OF_MEMORY_MANAGER_BEFORE_HEAP as *mut PhysicalMemoryManager;
            (*result) = PhysicalMemoryManager {
                MemoryMap: memoryMap,
                Blobs: from_fn(|_| MemoryBlob::default()),
            };

            (*result).Reserve(
                ADDRESS_OF_MEMORY_MANAGER_BEFORE_HEAP,
                size_of::<PhysicalMemoryManager>(),
                false,
            );

            result
        }
    }

    pub(crate) fn Reserve(
        &mut self,
        requestLocation: usize,
        requestAmmount: usize,
        allowReserved: bool,
    ) {
        let requestLocation = requestLocation;
        let requestAmmount = requestAmmount;

        for index in 0..(self.MemoryMap.Count as usize) {
            let memoryType = self.MemoryMap.Entries[index].GetType();

            let memoryMapBase = self.MemoryMap.Entries[index].BaseAddr;
            let memoryMapBase: Result<usize, _> = memoryMapBase.try_into();

            let memoryMapLength = self.MemoryMap.Entries[index].Length;
            let memoryMapLength: Result<usize, _> = memoryMapLength.try_into();

            if matches!(memoryMapBase, Err(_)) || matches!(memoryMapLength, Err(_)) {
                continue;
            }

            let memoryMapBase = memoryMapBase.unwrap();
            let memoryMapLength = memoryMapLength.unwrap();

            // Does this request to reserve fit in this memroy range?
            if (memoryMapBase <= requestLocation)
                && ((requestLocation + requestAmmount) <= (memoryMapBase + memoryMapLength))
            {
                match memoryType {
                    MemoryMapEntryType::AddressRangeMemory => {
                        self.ReserveInternal(
                            requestLocation,
                            requestAmmount,
                            memoryMapBase,
                            memoryMapLength,
                        );
                        return;
                    }
                    MemoryMapEntryType::AddressRangeReserved if allowReserved => {
                        self.ReserveInternal(
                            requestLocation,
                            requestAmmount,
                            memoryMapBase,
                            memoryMapLength,
                        );
                        return;
                    }
                    _ => {
                        self.Dump();
                        haltLoopWithMessage!(
                            "0x{:X} is in a {:?} region. Cannot use.",
                            requestLocation,
                            memoryType
                        );
                    }
                }
            }
        }

        let end = requestLocation + requestAmmount;
        vgaWriteLine!(
            "0x{:X}..0x{:X} for 0x{:X} not in memory range (of any type)",
            requestLocation,
            end,
            requestAmmount
        );

        self.Dump();
        haltLoop();
    }

    fn ReserveInternal(
        &mut self,
        requestLocation: usize,
        requestAmmount: usize,
        memoryMapBase: usize,
        memoryMapLength: usize,
    ) {
        let blobs = addr_of_mut!(self.Blobs);
        let mut nextIndex = 0;

        unsafe {
            for blobIndex in 0..(blobs.read_unaligned().len()) {
                nextIndex = blobIndex;
                let blobAddress = blobs.read_unaligned()[blobIndex].Address;
                let blobLength = blobs.read_unaligned()[blobIndex].Length;

                if blobLength == 0 {
                    // Can't reserve 0 bytes, so use that as the marker of used or not
                    // We've made it to the end without finding it is already being used
                    break;
                }

                // Is this request to reserved already reserved by something else?
                if requestLocation < (blobAddress + blobLength)
                    && (blobAddress) < (requestLocation + requestAmmount)
                {
                    vgaWriteLine!(
                        "0x{:X} for 0x{:X} overlaps with index {} 0x{:X} for 0x{:X}",
                        requestLocation,
                        requestAmmount,
                        blobIndex,
                        blobAddress,
                        blobLength,
                    );
                    haltLoop();
                }
            }

            blobs.read_unaligned()[nextIndex].Address = requestLocation;
            blobs.read_unaligned()[nextIndex].Length = requestAmmount;
        }

        vgaWriteLine!(
            "Reserved 0x{:X} bytes @ 0x{:X} within 0x{:X}..0x{:X} index {}",
            requestAmmount,
            requestLocation,
            memoryMapBase,
            memoryMapBase + memoryMapLength,
            nextIndex,
        );
    }

    pub(crate) fn Dump(&self) {
        for index in 0..(self.MemoryMap.Count as usize) {
            let memoryType = self.MemoryMap.Entries[index].GetType();

            let memoryMapBase = self.MemoryMap.Entries[index].BaseAddr;
            let memoryMapBase: Result<usize, _> = memoryMapBase.try_into();

            let memoryMapLength = self.MemoryMap.Entries[index].Length;
            let memoryMapLength: Result<usize, _> = memoryMapLength.try_into();

            if matches!(memoryMapBase, Err(_)) || matches!(memoryMapLength, Err(_)) {
                continue;
            }

            let memoryMapBase = memoryMapBase.unwrap();
            let memoryMapLength = memoryMapLength.unwrap();
            let memoryEnd = memoryMapBase + memoryMapLength;

            vgaWriteLine!(
                "0x{:X}..0x{:X} is {:?}",
                memoryMapBase,
                memoryEnd,
                memoryType
            );
        }
    }
}
