use core::any::type_name;

use crate::{
    assemblyStuff::halt::haltLoop,
    haltLoopWithMessage, loggerWrite, loggerWriteLine,
    memory::{
        map::MemoryMap,
        mapEntry::{MemoryMapEntry, MemoryMapEntryType},
    },
    memoryHelpers::{alignUp, zeroMemory2},
    memoryTypes::PhysicalAddressPlain,
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
    YoLo, // Allocate even if it isn't in the map. Using this for hardware IO.
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

        loggerWriteLine!(
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

            loggerWriteLine!(
                "0x{:X}..0x{:X} is {:?}",
                memoryMapBase,
                memoryEnd,
                memoryType
            );
        }
    }

    pub fn FindEntryForAddress(&self, address: usize) -> MemoryMapEntry {
        for index in 0..(self.MemoryMap.EntryCount as usize) {
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

            if address >= memoryMapBase && address < memoryEnd {
                return self.MemoryMap.Entries[index];
            }
        }

        haltLoopWithMessage!("0x{:X} not in any entry", address);
    }

    pub fn ReserveWhereverZeroed<T>(&mut self, sizeInBytes: usize, alignment: usize) -> *mut T {
        let nextBlob = self.nextFreeBlob();
        if None == nextBlob {
            haltLoopWithMessage!("No more blobs to store data in");
        }

        let nextBlob = nextBlob.unwrap();

        // This is a very dumb 'next available' algorithm as if previous allocations left gaps
        // this request would fit in, we'd end up skipping over them
        let mut nextAvailableAddress = None;

        for x in 0..nextBlob {
            let endAddress = self.Blobs[x].PhysicalAddress.address + self.Blobs[x].Length;
            if let Some(currentLowestAddress) = nextAvailableAddress {
                if endAddress > currentLowestAddress {
                    nextAvailableAddress = Some(endAddress);
                }
            } else {
                nextAvailableAddress = Some(endAddress);
            }
        }

        if nextAvailableAddress == None {
            // Not sure why we used to do this:
            //nextAvailableAddress = Some(0);

            // Going to halt for now
            haltLoopWithMessage!("No addresses available to allocate from");
        }

        let lowestAvailableAddress = alignUp(nextAvailableAddress.unwrap(), alignment);
        loggerWriteLine!(
            "Using blob {} to hold address 0x{:X}",
            nextBlob,
            lowestAvailableAddress
        );

        let entry = self.FindEntryForAddress(lowestAvailableAddress);
        loggerWrite!("The address is in entry: ");
        entry.dumpEx(true);

        let rt: bool = entry.fits(lowestAvailableAddress, sizeInBytes);

        if !rt {
            loggerWriteLine!(
                "The address cannot fit in the entry; update this code to look elsewhere"
            );
            self.MemoryMap.dumpEx(true);
            haltLoopWithMessage!("Couldn't find anywhere for 0x{:X} bytes", sizeInBytes);
        }

        let result = lowestAvailableAddress as *mut T;
        loggerWriteLine!("Zeroing 0x{:X} bytes at 0x{:X}", sizeInBytes, lowestAvailableAddress);
        unsafe {
            zeroMemory2(result);
        }

        self.Blobs[nextBlob].PhysicalAddress.address = lowestAvailableAddress;
        self.Blobs[nextBlob].Length = sizeInBytes;

        loggerWriteLine!("Allocation complete");

        return result;
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
