use super::memoryMap::{MemoryMap, MemoryMapEntryType};
use crate::{assemblyHelpers::breakpoint::HaltLoop, vgaWriteLine};
use core::{fmt::Write, mem::size_of, ptr::null_mut};

pub struct DumbHeap {
    MemoryMap: MemoryMap,
    Index: usize,
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
        for index in 0..(memoryMap.Count as usize) {
            if let MemoryMapEntryType::AddressRangeMemory = memoryMap.Entries[index].GetType() {
                let stupidAddress = memoryMap.Entries[index].BaseAddr as usize;
                let stupidSize = memoryMap.Entries[index].Length as usize;
                if stupidSize < (1 * 1024 * 1024) {
                    vgaWriteLine!(
                        "0x{:X} is too small at 0x{:X} bytes",
                        stupidAddress,
                        stupidSize
                    );
                } else {
                    vgaWriteLine!("0x{:X} wins at 0x{:X} bytes", stupidAddress, stupidSize);
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
                        MemoryMap: memoryMap,
                        Index: index,
                        First: heapEntryAddress,
                    };
                }
            }
        }

        vgaWriteLine!("Couldn't find any useable memory!");
        HaltLoop();
    }

    pub fn DoSomething(&mut self, ammount: usize) -> usize {
        unsafe {
            if (*self.First).Free {
                if ammount <= (*self.First).Size {
                    if (*self.First).Next != null_mut() {
                        vgaWriteLine!("Next poitner is already populated...");
                        HaltLoop();
                    } else {
                        let headerSize = size_of::<HeapEntry>();
                        let remainingAfterHeader = (*self.First).Size - ammount - headerSize;
                        // BUGBUG: Deal with some underflow or whatever this warning is reffering to
                        if remainingAfterHeader >= 0 {
                            vgaWriteLine!(
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
                            writePointer.write(nextEntry);

                            (*self.First).Size = ammount;
                            (*self.First).Next = writePointer;
                            (*self.First).Free = false;

                            return (*self.First).Address;
                        } else {
                            vgaWriteLine!("Allocation wouldn't leave room for next pointer");
                            HaltLoop();
                        }
                    }
                } else {
                    vgaWriteLine!("You want too much");
                    HaltLoop();
                }
            } else {
                vgaWriteLine!("Don't know how to grow mem list yet");
                HaltLoop();
            }
        }
    }

    fn NeededSize(requested: u64) -> usize {
        requested as usize + size_of::<HeapEntry>()
    }

    pub fn DumpHeap(&mut self) {
        let mut current = self.First;
        while current != null_mut() {
            unsafe {
                let currentAddress = current as usize;
                let currentItem = current.read();
                let nextAddress = currentItem.Next as usize;
                let free = currentItem.Free;
                vgaWriteLine!(
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
