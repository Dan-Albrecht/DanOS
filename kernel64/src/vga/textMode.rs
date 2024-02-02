const VGA_WIDTH: u16 = 80;
const VGA_HEIGHT: u16 = 25;

const VGA_ADDRESS_PORT: u16 = 0x3D4;
const VGA_DATA_PORT: u16 = 0x3D5;
const CURSOR_HIGH_REG: u8 = 0xE;
const CURSOR_LOW_REG: u8 = 0xF;
const VGA_BUFFER_ADDRESS: u32 = 0xB8000;

use core::fmt::Write;

use crate::assemblyHelpers::ports::{inB, outB};

pub struct VgaHelper;
impl Write for VgaHelper {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        writeString(s.as_bytes());
        Ok(())
    }
}

#[macro_export]
macro_rules! vgaWrite {
    ($($arg:tt)*) => {
        let mut x: Foo = Foo;
        let _ = write!(&mut x, $($arg)*);
    };
}

#[macro_export]
macro_rules! vgaWriteLine {
    ($($args:tt)*) => {
        let mut x: VgaHelper = VgaHelper;
        let _ = write!(&mut x, $($args)*);
        let _ = write!(&mut x, "\r\n");
    };
}

pub struct CursorPosition {
    pub x: u8,
    pub y: u8,
}

pub unsafe fn scrollUp() {
    let vgaBuffer = VGA_BUFFER_ADDRESS as *mut u8;

    for row in 1..VGA_HEIGHT {
        for column in 0..VGA_WIDTH {
            // Each character on the screen takes 2 bytes (color+character)
            let sourceOffset = (row * VGA_WIDTH + column) * 2;
            let destinationOffset = ((row - 1) * VGA_WIDTH + column) * 2;

            // Character
            *vgaBuffer.offset(destinationOffset as isize) =
                *vgaBuffer.offset(sourceOffset as isize);

            // Color
            *vgaBuffer.offset(destinationOffset as isize + 1) =
                *vgaBuffer.offset(sourceOffset as isize + 1);
        }
    }

    // Clear the last row as we've scrolled it up now
    for column in 0..VGA_WIDTH {
        let row = VGA_HEIGHT - 1;
        let destinationOffset = (row * VGA_WIDTH + column) * 2;

        *vgaBuffer.offset(destinationOffset as isize) = 0;

        // Assign a default color so if the cursor is blinking here you can see it
        *vgaBuffer.offset(destinationOffset as isize + 1) = 7;
    }
}

fn writeString(msg: &[u8]) {
    let vgaBuffer = VGA_BUFFER_ADDRESS as *mut u8;
    let mut cursorPosition = getCursorPosition();

    for (_i, &byte) in msg.iter().enumerate() {
        unsafe {
            if byte == b'\r' {
                cursorPosition.x = 0;
            } else if byte == b'\n' {
                if cursorPosition.y == 24 {
                    scrollUp();
                } else {
                    cursorPosition.y += 1;
                }
            } else {
                let currentOffset = calculatedOffset(&cursorPosition);

                *vgaBuffer.offset(currentOffset) = byte;
                *vgaBuffer.offset(currentOffset + 1) = 0x74; // Red on gray

                cursorPosition.x += 1;
            }
        }
    }

    unsafe {
        setCursorPosition(&cursorPosition);
    }
}

fn calculatedOffset(cursorPosition: &CursorPosition) -> isize {
    let mut result = cursorPosition.y as u16;
    result *= VGA_WIDTH;
    result += cursorPosition.x as u16;
    result *= 2; // Each character takes up 2 bytes in the buffer

    return result as isize;
}

pub fn getCursorPosition() -> CursorPosition {
    unsafe {
        outB(VGA_ADDRESS_PORT, CURSOR_HIGH_REG);
        let mut position = inB(VGA_DATA_PORT) as u16;
        position <<= 8; // Move to high byte.

        outB(VGA_ADDRESS_PORT, CURSOR_LOW_REG);
        position |= inB(VGA_DATA_PORT) as u16;

        let x = (position % VGA_WIDTH) as u8;
        let y = (position / VGA_WIDTH) as u8;

        CursorPosition { x, y }
    }
}

pub unsafe fn setCursorPosition(pos: &CursorPosition) {
    let mut positionOffset: u16 = pos.y as u16;
    positionOffset *= VGA_WIDTH;
    positionOffset += pos.x as u16;

    outB(VGA_ADDRESS_PORT, CURSOR_HIGH_REG);
    outB(VGA_DATA_PORT, (positionOffset >> 8) as u8);

    outB(VGA_ADDRESS_PORT, CURSOR_LOW_REG);
    outB(VGA_DATA_PORT, positionOffset as u8);
}
