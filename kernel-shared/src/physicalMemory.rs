use core::any::type_name;

use crate::{
    assemblyStuff::halt::haltLoop,
    haltLoopWithMessage, loggerWriteLine,
    memory::{
        map::MemoryMap,
        mapEntry::{MemoryMapEntry, MemoryMapEntryType},
    },
    memoryHelpers::{alignUp, zeroMemory},
    memoryTypes::PhysicalAddressPlain,
};

pub struct PhysicalMemoryManager {
    pub MemoryMap: MemoryMap,
    pub Blobs: [MemoryBlob; 0x10],
}

pub struct MemoryBlob {
    PhysicalAddress: PhysicalAddressPlain,
    Length: usize,
    What: [u8; 0x40], // Short description of what this is used for
}

// BUGUBG: Come up with a better name
#[derive(Debug)]
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
            What: [0; 0x40],
        }
    }
}

impl PhysicalMemoryManager {
    pub fn Reserve(
        &mut self,
        forWhat: &str,
        requestLocation: usize,
        requestAmmount: usize,
        whatDo: WhatDo,
    ) {
        loggerWriteLine!(
            "Reserving 0x{:X} bytes @ 0x{:X} for {} via method {:?}",
            requestAmmount,
            requestLocation,
            forWhat,
            whatDo
        );

        if let WhatDo::YoLo = whatDo {
            self.ReserveInternal(requestLocation, requestAmmount, forWhat);
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
                            self.ReserveInternal(requestLocation, requestAmmount, forWhat);
                            return;
                        }
                        MemoryMapEntryType::AddressRangeReserved
                            if let WhatDo::UseReserved = whatDo =>
                        {
                            self.ReserveInternal(requestLocation, requestAmmount, forWhat);
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

    fn ReserveInternal(&mut self, requestLocation: usize, requestAmmount: usize, forWhat: &str,) {
        // Figure out if we have room for this
        let firstFreeIndex = self.nextFreeBlob();

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

        let bytes = forWhat.as_bytes();
        let copy_len = core::cmp::min(bytes.len(), self.Blobs[firstFreeIndex].What.len());
        self.Blobs[firstFreeIndex].What[..copy_len].copy_from_slice(&bytes[..copy_len]);

        loggerWriteLine!(
            "Reserved 0x{:X} bytes @ 0x{:X} index {} for {}",
            requestAmmount,
            requestLocation,
            firstFreeIndex,
            forWhat
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

    pub fn DumpBlobs(&self) {
        for index in 0..self.Blobs.len() {
            if self.Blobs[index].Length == 0 {
                break;
            }

            loggerWriteLine!(
                "Blob {}: 0x{:X} for 0x{:X} for {}",
                index,
                self.Blobs[index].PhysicalAddress.address,
                self.Blobs[index].Length,
                core::str::from_utf8(&self.Blobs[index].What).unwrap_or(""),
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

    pub fn ReserveWhereverZeroed2<T>(&mut self) -> *mut T {
        let sizeInBytes = core::mem::size_of::<T>();
        let alignment = core::mem::align_of::<T>();
        let result = self.ReserveWhereverZeroed(type_name::<T>(), sizeInBytes, alignment);
        let result = result as *mut T;

        return result;
    }

    pub fn ReserveWhereverZeroed(
        &mut self,
        forWhat: &str,
        sizeInBytes: usize,
        alignment: usize,
    ) -> usize {
        loggerWriteLine!(
            "Reserving and zeroing 0x{:X} bytes for {} with alignment 0x{:X}",
            sizeInBytes,
            forWhat,
            alignment
        );

        let nextBlob = self.nextFreeBlob();
        if None == nextBlob {
            haltLoopWithMessage!("No more blobs to store data in");
        }

        let nextBlob = nextBlob.unwrap();
        let mut candidateAddress: usize;

        for x in 0..self.MemoryMap.EntryCount {
            // Entry isn't useful if isn't a memory range
            if self.MemoryMap.Entries[x as usize].getType()
                != MemoryMapEntryType::AddressRangeMemory
            {
                continue;
            }

            candidateAddress = alignUp(
                self.MemoryMap.Entries[x as usize].BaseAddress as usize,
                alignment,
            );

            loop {
                loggerWriteLine!("Checking 0x{:X} for 0x{:X}", candidateAddress, sizeInBytes);

                // Dumb check to see if it'll fit at all (nermind if might already be used)
                if !self.MemoryMap.Entries[x as usize].fits(candidateAddress as usize, sizeInBytes)
                {
                    break;
                }

                if let Some(blobUsing) = self.findBlobUsing(candidateAddress as usize, sizeInBytes)
                {
                    candidateAddress = self.Blobs[blobUsing].PhysicalAddress.address
                        + self.Blobs[blobUsing].Length;
                    candidateAddress = alignUp(candidateAddress, alignment);
                    continue;
                }

                self.Blobs[nextBlob].PhysicalAddress.address = candidateAddress as usize;
                self.Blobs[nextBlob].Length = sizeInBytes;

                let bytes = forWhat.as_bytes();
                let copy_len = core::cmp::min(bytes.len(), self.Blobs[nextBlob].What.len());
                self.Blobs[nextBlob].What[..copy_len].copy_from_slice(&bytes[..copy_len]);

                unsafe {
                    loggerWriteLine!("Zeroing 0x{:X} for 0x{:X}", candidateAddress, sizeInBytes);
                    zeroMemory(candidateAddress, sizeInBytes);
                    loggerWriteLine!("Zeroing complete");
                }

                return candidateAddress;
            }
        }

        haltLoopWithMessage!("Can't find a free blob");
    }

    fn nextFreeBlob(&self) -> Option<usize> {
        for index in 0..self.Blobs.len() {
            if self.Blobs[index].Length == 0 {
                loggerWriteLine!("Next free blob is {}", index);
                return Some(index);
            }

            loggerWriteLine!(
                "Blob {} is using address 0x{:X}",
                index,
                self.Blobs[index].PhysicalAddress.address
            );
        }

        None
    }

    fn findBlobUsing(&self, address: usize, size: usize) -> Option<usize> {
        for index in 0..self.Blobs.len() {
            let blobSize = self.Blobs[index].Length;

            // Once we hit an unused blob, nothing else used will follow so we know we're not overlapping anywhere
            if blobSize == 0 {
                loggerWriteLine!("No blobs are using address 0x{:X}", address);
                return None;
            }

            let blobStart = self.Blobs[index].PhysicalAddress.address;
            let blobEnd = blobStart + blobSize - 1;

            let addressEnd = address + size - 1;

            // Check for any overlap between ranges using standard range overlap formula
            if address <= blobEnd && blobStart <= addressEnd {
                loggerWriteLine!(
                    "Blob {} overlaps with address range 0x{:X}-0x{:X}",
                    index,
                    address,
                    addressEnd
                );
                return Some(index);
            }
        }

        None
    }
}
