#![allow(non_snake_case)]

use kernel_shared::magicConstants::{RANDOM_EVEN_MEGABYTE, RANDOM_ODD_MEGABYTE};

pub unsafe fn IsTheA20LineEnabled() -> bool {
    // Write to an even and odd megabyte address
    // If A20 is disabled, the memory will wrap around
    // and we'll write to the same location and the final
    // values will be the same.
    let oddMegabyte = RANDOM_ODD_MEGABYTE as *mut u16;
    let evenMegabyte = RANDOM_EVEN_MEGABYTE as *mut u16;

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

    *oddMegabyte = oldOddValue;
    *evenMegabyte = oldEvenValue;

    result
}
