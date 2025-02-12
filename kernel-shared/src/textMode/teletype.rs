#[cfg(feature = "use_bios")]
use core::arch::asm;

#[cfg(feature = "use_bios")]
pub fn printChar(char: u8) {
    unsafe {
        asm!(
            "mov ah, 0x0E", // Teletype output function
            "xor bx, bx",   // BH = page number (0), BL is N/A for this mode
                            // so 0 it for consistency
            "int 0x10",     // Video Services
            out("ah") _,
            out("bx") _,
            in("al") char,  // Char to print
        );
    }
}

#[cfg(feature = "use_bios")]
pub fn printLine(blah: &[u8]) {
    for b in blah {
        printChar(*b);
    }

    printChar(b'\r');
    printChar(b'\n');
}

#[cfg(feature = "use_bios")]
pub fn printUsizeLine(num: usize) {
    let mut num = num;
    let mut buffer = [0u8; 20];
    let mut i = 0;

    while num > 0 {
        buffer[i] = (num % 10) as u8 + b'0';
        num /= 10;
        i += 1;
    }

    while i > 0 {
        i -= 1;
        printChar(buffer[i]);
    }

    printChar(b'\r');
    printChar(b'\n');
}
