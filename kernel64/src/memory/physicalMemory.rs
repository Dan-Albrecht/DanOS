use core::fmt::Write;
use kernel_shared::{assemblyStuff::halt::haltLoop, haltLoopWithMessage};

use crate::loggerWriteLine;

use super::{
    memoryMap::{MemoryMap, MemoryMapEntryType},
    virtualMemory::WhatDo,
};

pub struct PhysicalMemoryManager {
    pub MemoryMap: MemoryMap,
    pub Blobs: [MemoryBlob; 0xF],
}

pub struct MemoryBlob {
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
    pub(crate) fn Reserve(
        &mut self,
        requestLocation: usize,
        requestAmmount: usize,
        whatDo: WhatDo,
    ) {
        let requestLocation = requestLocation;
        let requestAmmount = requestAmmount;

        if let WhatDo::YoLo = whatDo {
            self.ReserveInternal(
                requestLocation,
                requestAmmount,
                requestLocation,
                requestAmmount,
            );
            return;
        } else {
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
                        MemoryMapEntryType::AddressRangeReserved
                            if let WhatDo::UseReserved = whatDo =>
                        {
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
            loggerWriteLine!(
                "0x{:X}..0x{:X} for 0x{:X} not in memory range (of any type)",
                requestLocation,
                end,
                requestAmmount
            );

            self.Dump();
            haltLoop();
        }
    }

    fn ReserveInternal(
        &mut self,
        requestLocation: usize,
        requestAmmount: usize,
        memoryMapBase: usize,
        memoryMapLength: usize,
    ) {
        let mut nextIndex = 0;

        // BUGBUG: This code doesn't correctly handle when the array is full
        for blobIndex in 0..(self.Blobs.len()) {
            nextIndex = blobIndex;
            let blobAddress = self.Blobs[blobIndex].Address;
            let blobLength = self.Blobs[blobIndex].Length;

            if blobLength == 0 {
                // Can't reserve 0 bytes, so use that as the marker of used or not
                // We've made it to the end without finding it is already being used
                break;
            }

            // Is this request to reserved already reserved by something else?
            if requestLocation < (blobAddress + blobLength)
                && (blobAddress) < (requestLocation + requestAmmount)
            {
                loggerWriteLine!(
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

        self.Blobs[nextIndex].Address = requestLocation;
        self.Blobs[nextIndex].Length = requestAmmount;

        loggerWriteLine!(
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

            loggerWriteLine!(
                "0x{:X}..0x{:X} is {:?}",
                memoryMapBase,
                memoryEnd,
                memoryType
            );
        }
    }

    // BUGBUG: This is very dumb and inefficent
    pub(crate) fn ReserveWherever<T>(&mut self, sizeInBytes: usize) -> *mut T {
        let mut start = 0;
        let len = self.Blobs.len();

        for blobIndex in 0..len {
            let x = self.Blobs[blobIndex].Address;
            let y = self.Blobs[blobIndex].Length;

            if y == 0 {
                // 0 length is our current 'not used' marker
                break;
            }

            let z = x + y;
            start = z;
        }

        self.Reserve(start, sizeInBytes, WhatDo::Normal);

        return start as *mut T;
    }
}
