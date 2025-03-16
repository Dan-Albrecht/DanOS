use core::any::type_name;

use crate::{
    assemblyStuff::halt::haltLoop,
    haltLoopWithMessage,
    memory::{map::MemoryMap, mapEntry::MemoryMapEntryType},
    memoryHelpers::{alignUp, zeroMemory2},
    memoryTypes::PhysicalAddressPlain,
    vgaWriteLine,
};

pub struct PhysicalMemoryManager {
    pub MemoryMap: MemoryMap,
    pub Blobs: [MemoryBlob; 0xF],
}

pub struct MemoryBlob {
    PhysicalAddress: PhysicalAddressPlain,
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
            PhysicalAddress: PhysicalAddressPlain { address: 0 },
            Length: 0,
        }
    }
}

impl PhysicalMemoryManager {
    pub fn Reserve(&mut self, requestLocation: usize, requestAmmount: usize, whatDo: WhatDo) {
        if let WhatDo::YoLo = whatDo {
            self.ReserveInternal(requestLocation, requestAmmount);
            return;
        } else {
            for index in 0..(self.MemoryMap.EntryCount as usize) {
                let memoryType = self.MemoryMap.Entries[index].getType();

                let memoryMapBase = self.MemoryMap.Entries[index].BaseAddress;
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
                            self.ReserveInternal(requestLocation, requestAmmount);
                            return;
                        }
                        MemoryMapEntryType::AddressRangeReserved
                            if let WhatDo::UseReserved = whatDo =>
                        {
                            self.ReserveInternal(requestLocation, requestAmmount);
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

    fn ReserveInternal(&mut self, requestLocation: usize, requestAmmount: usize) {
        // Figure out if we have room for this
        let mut firstFreeIndex = None;
        for blobIndex in 0..(self.Blobs.len()) {
            if self.Blobs[blobIndex].Length == 0 {
                firstFreeIndex = Some(blobIndex);
                break;
            }
        }

        if firstFreeIndex == None {
            haltLoopWithMessage!("No more room in {}", type_name::<PhysicalMemoryManager>());
        }

        let firstFreeIndex = firstFreeIndex.unwrap();
        let requestEnd = requestLocation + requestAmmount;

        // Check them to see if we overlap
        for blobIndex in 0..firstFreeIndex {
            let blobLocation = self.Blobs[blobIndex].PhysicalAddress.address;
            let blobAmmount = self.Blobs[blobIndex].Length;
            let blobEnd = blobLocation + blobAmmount;

            if requestLocation < blobEnd && blobLocation < requestEnd {
                haltLoopWithMessage!(
                    "0x{:X} for 0x{:X} overlaps with index {} 0x{:X} for 0x{:X}",
                    requestLocation,
                    requestAmmount,
                    blobIndex,
                    blobLocation,
                    blobAmmount,
                );
            }
        }

        self.Blobs[firstFreeIndex].PhysicalAddress = PhysicalAddressPlain {
            address: requestLocation,
        };
        self.Blobs[firstFreeIndex].Length = requestAmmount;

        vgaWriteLine!(
            "Reserved 0x{:X} bytes @ 0x{:X} index {}",
            requestAmmount,
            requestLocation,
            firstFreeIndex,
        );
    }

    pub fn Dump(&self) {
        for index in 0..(self.MemoryMap.EntryCount as usize) {
            let memoryType = self.MemoryMap.Entries[index].getType();

            let memoryMapBase = self.MemoryMap.Entries[index].BaseAddress;
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

    pub fn ReserveWhereverZeroed<T>(&mut self, sizeInBytes: usize, alignment: usize) -> *mut T {
        let nextBlob = self.nextFreeBlob();
        if None == nextBlob {
            haltLoopWithMessage!("No more blobs to store data in");
        }

        let firstFreeIndex = nextBlob.unwrap();
        let mut lowestAvailableAddress = None;

        for x in 0..firstFreeIndex {
            let endAddress = self.Blobs[x].PhysicalAddress.address + self.Blobs[x].Length;
            if let Some(currentLowestAddress) = lowestAvailableAddress {
                if endAddress > currentLowestAddress {
                    lowestAvailableAddress = Some(endAddress);
                }
            } else {
                lowestAvailableAddress = Some(endAddress);
            }
        }

        if lowestAvailableAddress == None {
            lowestAvailableAddress = Some(0);
        }

        let mut lowestAvailableAddress = alignUp(lowestAvailableAddress.unwrap(), alignment) as u64;
        let sizeInBytes = sizeInBytes as u64;

        // See where this will fit
        // BUGBUG: We assume memory map is in ascending order, not sure if anything guarntees that
        // BUGBUG: This number casting is out of control...
        for x in 0..self.MemoryMap.EntryCount as usize {
            let entry = self.MemoryMap.Entries[x];
            if entry.getType() == MemoryMapEntryType::AddressRangeMemory {
                if lowestAvailableAddress >= entry.BaseAddress
                    && lowestAvailableAddress <= entry.BaseAddress + entry.Length
                {
                    // The start is within the range, but what about the end?
                    let requestEnd = lowestAvailableAddress + sizeInBytes;
                    if requestEnd >= entry.BaseAddress
                        && requestEnd <= entry.BaseAddress + entry.Length
                    {
                        self.Reserve(
                            lowestAvailableAddress as usize,
                            sizeInBytes as usize,
                            WhatDo::Normal,
                        );
                        let result = lowestAvailableAddress as *mut T;
                        unsafe {
                            zeroMemory2(result);
                        }
                        return result;
                    } else {
                        vgaWriteLine!("End goes past the end of this blob, trying next...");
                        lowestAvailableAddress = alignUp(
                            entry.BaseAddress as usize + entry.Length as usize + 1 as usize,
                            alignment,
                        ) as u64;
                    }
                } else if lowestAvailableAddress < entry.BaseAddress {
                    let potentialStart = alignUp(entry.BaseAddress as usize, alignment);
                    let potentialEnd = potentialStart + sizeInBytes as usize;

                    if potentialStart < entry.BaseAddress as usize + entry.Length as usize
                        && entry.BaseAddress < potentialEnd as u64
                    {
                        self.Reserve(
                            potentialStart as usize,
                            sizeInBytes as usize,
                            WhatDo::Normal,
                        );
                        let result = potentialStart as *mut T;
                        unsafe {
                            zeroMemory2(result);
                        }
                        return result;
                    }
                }
            }
        }

        self.MemoryMap.dump();
        haltLoopWithMessage!("Couldn't find anywhere for 0x{:X} bytes", sizeInBytes);
    }

    fn nextFreeBlob(&self) -> Option<usize> {
        for index in 0..self.Blobs.len() {
            if self.Blobs[index].Length == 0 {
                return Some(index);
            }
        }

        None
    }
}
