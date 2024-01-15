#![allow(non_snake_case)]

pub unsafe fn IsTheA20LineEnabled() -> bool {
    // Write to an even and odd megabyte address
    // If A20 is disabled, the memory will wrap around
    // and we'll write to the same location and the final
    // values will be the same.
    let oddMegabyte = 0x112345 as *mut u16;
    let evenMegabyte = 0x012345 as *mut u16;

    *oddMegabyte = 123;
    *evenMegabyte = 456;

    if *oddMegabyte != *evenMegabyte {
        return true;
    } else {
        return false;
    }
}
