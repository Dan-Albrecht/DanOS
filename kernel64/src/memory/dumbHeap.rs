use kernel_shared::{
    assemblyStuff::halt::haltLoop, haltLoopWithMessage, memory::{map::MemoryMap, mapEntry::MemoryMapEntryType}, memoryHelpers::alignUp, memoryTypes::{PhysicalAddress, VirtualAddress}
};

use crate::loggerWriteLine;
use core::{array::from_fn, mem::{size_of, align_of}, ptr::null_mut};

use super::memoryStuff::MemoryStuff;

pub struct BootstrapDumbHeap {
    Entries: [BootstrapDumbHeapEntry; 10],
    StartAddress: usize,
    Length: usize,
    VirtualIsGreaterThanPhysical: bool,
    Adjustment: usize,
}

struct BootstrapDumbHeapEntry {
    Address: usize,
    Length: usize,
}

impl Default for BootstrapDumbHeapEntry {
    fn default() -> Self {
        BootstrapDumbHeapEntry {
            Address: 0,
            Length: 0,
        }
    }
}

impl MemoryStuff for BootstrapDumbHeap {
    fn allocate<T>(&mut self) -> *mut T {
        let size = size_of::<T>();
        let align = align_of::<T>();
        let address : VirtualAddress<T> = BootstrapDumbHeap::allocate(self, size, align);
        let address = address.address;

        return address as *mut T;
    }

    fn free(&mut self, address: usize) {
        todo!()
    }
}

impl BootstrapDumbHeap {
    pub fn new(
        address: usize,
        length: usize,
        virtualIsGreaterThanPhysical: bool,
        adjustment: usize,
    ) -> BootstrapDumbHeap {
        BootstrapDumbHeap {
            StartAddress: address,
            Length: length,
            Entries: from_fn(|_| BootstrapDumbHeapEntry::default()),
            VirtualIsGreaterThanPhysical: virtualIsGreaterThanPhysical,
            Adjustment: adjustment,
        }
    }

    pub fn debugDump(&self) {
        loggerWriteLine!("BDH dump:");
        for x in 0..self.Entries.len()  {
            if self.Entries[x].Length == 0 {
                break;
            }

            loggerWriteLine!("{} = 0x{:X}", x, self.Entries[x].Address);
        }
    }

    pub fn allocate<T>(&mut self, length: usize, alignment: usize) -> VirtualAddress<T> {
        if length > self.Length {
            haltLoopWithMessage!(
                "Requested length of 0x{:X} is bigger than the entire heap of 0x{:X}",
                length,
                self.Length
            );
        }

        if length == 0 {
            haltLoopWithMessage!("Allocating 0 doesn't make sense...");
        }

        let mut firstFree = None;

        for index in 0..self.Entries.len() {
            // 0 length entries are not allowed, so that's how we indicate free instead of having another bool
            if self.Entries[index].Length == 0 {
                firstFree = Some(index);
                break;
            }
        }

        if firstFree == None {
            haltLoopWithMessage!("All dumb entries already taken");
        }

        let firstFree = firstFree.unwrap();
        let startAddress;

        if firstFree == 0 {
            startAddress = self.StartAddress;
        } else {
            startAddress = self.Entries[firstFree - 1].Address + self.Entries[firstFree - 1].Length;
        }

        let aligned = alignUp(startAddress, alignment);
        if aligned != startAddress {
            loggerWriteLine!("BDH aligned 0x{:X} to 0x{:X}", startAddress, aligned);
        }

        // Don't want to accidentally use the wrong value, so make them the same
        let startAddress = aligned;
        let endAddress = startAddress + length;
        let heapLimit = self.StartAddress + self.Length;

        if endAddress > heapLimit {
            haltLoopWithMessage!(
                "BDH is full. 0x{:X}..=0x{:X} is out of range of 0x{:X}..=0x{:X}",
                startAddress,
                endAddress,
                self.StartAddress,
                self.StartAddress + self.Length
            );
        }

        self.Entries[firstFree].Address = startAddress;
        self.Entries[firstFree].Length = length;

        VirtualAddress::new(startAddress)
    }

    pub fn vToP<T>(&self, address: &VirtualAddress<T>) -> PhysicalAddress<T> {
        let result;
        if self.VirtualIsGreaterThanPhysical {
            result = address.address - self.Adjustment
        } else {
            result = address.address + self.Adjustment
        }

        PhysicalAddress::new(result)
    }

    pub fn pToV<T>(&self, address: &PhysicalAddress<T>) -> VirtualAddress<T> {
        let result;
        if self.VirtualIsGreaterThanPhysical {
            result = address.address + self.Adjustment
        } else {
            result = address.address - self.Adjustment
        }

        VirtualAddress::new(result)
    }
}

pub struct DumbHeap {
    First: *mut HeapEntry,
}

struct HeapEntry {
    // Is this free?
    Free: bool,

    // The addres this starts to cover
    // This entry will always be at Address - sizeof(HeapEntry)
    Address: usize,

    // Space in bytes this entry covers
    Size: usize,

    // Pointer to next
    Next: *mut HeapEntry,
}

impl DumbHeap {
    pub fn new(memoryMap: MemoryMap) -> Self {
        for index in 0..(memoryMap.EntryCount as usize) {
            if let MemoryMapEntryType::AddressRangeMemory = memoryMap.Entries[index].getType() {
                let stupidAddress = memoryMap.Entries[index].BaseAddress as usize;
                let stupidSize = memoryMap.Entries[index].Length as usize;
                if stupidSize < (1 * 1024 * 1024) {
                    loggerWriteLine!(
                        "0x{:X} is too small at 0x{:X} bytes",
                        stupidAddress,
                        stupidSize
                    );
                } else {
                    loggerWriteLine!("0x{:X} wins at 0x{:X} bytes", stupidAddress, stupidSize);
                    let heapEntryAddress = stupidAddress as *mut HeapEntry;
                    let entry = HeapEntry {
                        Free: true,
                        Address: stupidAddress + size_of::<HeapEntry>(),
                        Size: stupidSize - size_of::<HeapEntry>(),
                        Next: null_mut(),
                    };

                    unsafe {
                        heapEntryAddress.write(entry);
                    }

                    return DumbHeap {
                        First: heapEntryAddress,
                    };
                }
            }
        }

        loggerWriteLine!("Couldn't find any useable memory!");
        haltLoop();
    }

    pub fn DoSomething(&mut self, ammount: usize) -> usize {
        unsafe {
            if (*self.First).Free {
                if ammount <= (*self.First).Size {
                    if (*self.First).Next != null_mut() {
                        loggerWriteLine!("Next poitner is already populated...");
                        haltLoop();
                    } else {
                        let headerSize = size_of::<HeapEntry>();
                        let totalNeeded = ammount + headerSize;

                        if totalNeeded <= (*self.First).Size {
                            let remainingAfterHeader = (*self.First).Size - totalNeeded;
                            loggerWriteLine!(
                                "Enough for another 0x{:X} byte entry",
                                remainingAfterHeader
                            );
                            let nextEntryAddress = (*self.First).Address + ammount;
                            let nextAddres = nextEntryAddress + headerSize;
                            let nextEntry = HeapEntry {
                                Free: true,
                                Address: nextAddres,
                                Size: remainingAfterHeader,
                                Next: null_mut(),
                            };

                            let writePointer = nextEntryAddress as *mut HeapEntry;
                            writePointer.write_unaligned(nextEntry);

                            (*self.First).Size = ammount;
                            (*self.First).Next = writePointer;
                            (*self.First).Free = false;

                            return (*self.First).Address;
                        } else {
                            loggerWriteLine!("Allocation wouldn't leave room for next pointer");
                            haltLoop();
                        }
                    }
                } else {
                    loggerWriteLine!("You want too much");
                    haltLoop();
                }
            } else {
                loggerWriteLine!("Don't know how to grow mem list yet");
                haltLoop();
            }
        }
    }

    pub fn DumpHeap(&mut self) {
        let mut current = self.First;
        while current != null_mut() {
            unsafe {
                let currentAddress = current as usize;
                let currentItem = current.read_unaligned();
                let nextAddress = currentItem.Next as usize;
                let free = currentItem.Free;
                loggerWriteLine!(
                    "Entry: 0x{:X} Free: {} Points: 0x{:X} For: 0x{:X} Next: 0x{:X}",
                    currentAddress,
                    free,
                    currentItem.Address,
                    currentItem.Size,
                    nextAddress
                );
                current = currentItem.Next;
            }
        }
    }
}
