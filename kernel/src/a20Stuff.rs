#![allow(non_snake_case)]

use kernel_shared::{
    assemblyStuff::halt::haltLoop, haltLoopWithMessage, memory::{map::MemoryMap, mapEntry::MemoryMapEntryType}, vgaWriteLine
};

pub unsafe fn IsTheA20LineEnabled(memoryMap: &MemoryMap) -> bool { unsafe {
    // Write to an even and odd megabyte address
    // If A20 is disabled, the memory will wrap around
    // and we'll write to the same location and the final
    // values will be the same.
    let mut evenMegabyte = 0x20_0000 as *mut u16;
    let mut oddMegabyte = 0x30_0000 as *mut u16;

    // Try a few times to find somewhere to write then give up
    for _ in 0..5 {
        if memoryMap.IsValid(
            (oddMegabyte as usize).try_into().unwrap(),
            size_of::<u16>().try_into().unwrap(),
            MemoryMapEntryType::AddressRangeMemory,
        ) && memoryMap.IsValid(
            (evenMegabyte as usize).try_into().unwrap(),
            size_of::<u16>().try_into().unwrap(),
            MemoryMapEntryType::AddressRangeMemory,
        ) {
            // Need to back up these values and restore as we may
            // have already loaded code here.
            // Learned that the hard way. :'(
            let oldOddValue = *oddMegabyte;
            *oddMegabyte = 123;

            let oldEvenValue = *evenMegabyte;
            *evenMegabyte = 456;

            let result: bool;

            if *oddMegabyte != *evenMegabyte {
                result = true;
            } else {
                result = false;
            }

            vgaWriteLine!(
                "Using 0x{:X} and 0x{:X} to test A20",
                evenMegabyte as usize,
                oddMegabyte as usize
            );
            *oddMegabyte = oldOddValue;
            *evenMegabyte = oldEvenValue;

            return result;
        }

        oddMegabyte = ((oddMegabyte as usize) + 0x20_0000) as *mut u16;
        evenMegabyte = ((evenMegabyte as usize) + 0x20_0000) as *mut u16;
    }

    haltLoopWithMessage!("Couldn't find a location to do A20 test");
}}
