use core::fmt::Write;

use crate::{
    assemblyStuff::halt::haltLoop,
    haltLoopWithMessage,
    memoryMap::{MemoryMap, MemoryMapEntryType},
    vgaWriteLine,
};

pub struct PhysicalMemoryManager {
    pub MemoryMap: MemoryMap,
    pub Blobs: [MemoryBlob; 0xF],
}

pub struct MemoryBlob {
    Address: usize,
    Length: usize,
}

// BUGUBG: Come up with a better name
pub enum WhatDo {
    Normal,
    UseReserved,
    YoLo, // Allocate even if it isn't in the map. Seeing this for hardware IO.
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
    pub fn Reserve(&mut self, requestLocation: usize, requestAmmount: usize, whatDo: WhatDo) {
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
            vgaWriteLine!(
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

        self.Blobs[nextIndex].Address = requestLocation;
        self.Blobs[nextIndex].Length = requestAmmount;

        vgaWriteLine!(
            "Reserved 0x{:X} bytes @ 0x{:X} within 0x{:X}..0x{:X} index {}",
            requestAmmount,
            requestLocation,
            memoryMapBase,
            memoryMapBase + memoryMapLength,
            nextIndex,
        );
    }

    pub fn Dump(&self) {
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

    fn isAddressInRange(&self, address: usize, memoryMapIndex: usize) -> bool {
        let entry = self.MemoryMap.Entries[memoryMapIndex];
        let start = entry.BaseAddr;
        let end = start + entry.Length;

        // 128-bit isn't coming anytime soon
        let address: u64 = address.try_into().unwrap();

        if address >= start && address <= end {
            true
        } else {
            false
        }
    }

    // BUGBUG: This is incomplete
    pub fn ReserveWherever<T>(&mut self, sizeInBytes: usize) -> *mut T {
        let nextBlob = self.nextFreeBlob();
        if None == nextBlob {
            haltLoopWithMessage!("No more blobs to store data in");
        }

        let firstFreeIndex = nextBlob.unwrap();
        let mut renameMe: Option<usize> = None;

        for x in 0..self.MemoryMap.Count as usize {
            if self.MemoryMap.Entries[x].GetType() == MemoryMapEntryType::AddressRangeMemory {
                // BUGBUG: We're currently trying to take the highest addresses as we know stack is below and unreserved
                // BUGBUG: We're making no attempt to avoid fragmentation, there could be a hole that could be filled by the request
                for y in 0..firstFreeIndex {
                    let toExamine = self.Blobs[y].Address;
                    if self.isAddressInRange(toExamine, x) {
                        if renameMe == None {
                            renameMe = Some(toExamine);
                        } else if let Some(currentRenam) = renameMe {
                            if toExamine < currentRenam {
                                renameMe = Some(toExamine);
                            }
                        }
                    }
                }

                // BUGBUG: We're just doing the first entry, we should check the rest instead of just failing
                // if the first one won't work
                if None == renameMe {
                    let mut start: usize = self.MemoryMap.Entries[x].BaseAddr.try_into().unwrap();
                    let length: usize = self.MemoryMap.Entries[x].Length.try_into().unwrap();
                    start += length;
                    start -= sizeInBytes;

                    self.Reserve(start, sizeInBytes, WhatDo::Normal);
                    return start as *mut T;
                } else {
                    let start: usize = renameMe.unwrap() - sizeInBytes;

                    self.Reserve(start, sizeInBytes, WhatDo::Normal);
                    return start as *mut T;
                }
            }
        }

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

    fn nextFreeBlob(&self) -> Option<usize> {
        for index in 0..self.Blobs.len() {
            if self.Blobs[index].Address == 0 {
                return Some(index);
            }
        }

        None
    }

    pub fn ReserveKernel32(&mut self, address: u64) {
        let firstEntry = self.MemoryMap.Entries[0];
        let ammount = (firstEntry.BaseAddr + firstEntry.Length) - address;
        let ammount: usize = ammount.try_into().unwrap();
        let address: usize = address.try_into().unwrap();
        self.Reserve(address, ammount, WhatDo::Normal);
    }
}
