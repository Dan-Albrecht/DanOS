use crate::{
    assemblyStuff::ports::{inB, outB},
    magicConstants::{VGA_BUFFER_ADDRESS, VGA_BYTES_PER_CHAR, VGA_HEIGHT, VGA_WIDTH},
};

const VGA_ADDRESS_PORT: u16 = 0x3D4;
const VGA_DATA_PORT: u16 = 0x3D5;
const CURSOR_HIGH_REG: u8 = 0xE;
const CURSOR_LOW_REG: u8 = 0xF;

#[repr(u8)]
#[allow(dead_code)]
enum ForegroundColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightMagenta = 13,
    Yellow = 14,
    White = 15,
}

#[repr(u8)]
#[allow(dead_code)]
enum BackgroundColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
}

fn getColorByte(fg: ForegroundColor, bg: BackgroundColor) -> u8 {
    (bg as u8) << 4 | (fg as u8)
}

struct CursorPosition {
    pub x: u8,
    pub y: u8,
}

pub fn scrollUp() {
    let vgaBuffer = VGA_BUFFER_ADDRESS as *mut u8;

    for row in 1..VGA_HEIGHT {
        for column in 0..VGA_WIDTH {
            let sourceOffset = (row * VGA_WIDTH + column) * VGA_BYTES_PER_CHAR;
            let destinationOffset = ((row - 1) * VGA_WIDTH + column) * VGA_BYTES_PER_CHAR;

            unsafe {
                // Character
                *vgaBuffer.offset(destinationOffset as isize) =
                    *vgaBuffer.offset(sourceOffset as isize);

                // Color
                *vgaBuffer.offset(destinationOffset as isize + 1) =
                    *vgaBuffer.offset(sourceOffset as isize + 1);
            }
        }
    }

    // Clear the last row as we've scrolled it up now
    for column in 0..VGA_WIDTH {
        let row = VGA_HEIGHT - 1;
        let destinationOffset = (row * VGA_WIDTH + column) * VGA_BYTES_PER_CHAR;

        unsafe {
            *vgaBuffer.offset(destinationOffset as isize) = 0;

            // Assign a default color so if the cursor is blinking here you can see it
            *vgaBuffer.offset(destinationOffset as isize + 1) = 7;
        }
    }
}

pub fn writeString(msg: &[u8]) {
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
            } else if byte == 0 {
                break;
            } else {
                // BUGBUG: This is another thing that doesn't make sense. The -1 isn't needed. But the side effect of it
                // is we're booting farther than we would otherwise. So take this hack now while we try to figure out the
                // real problem.
                if cursorPosition.x == (VGA_WIDTH as u8 - 1) {
                    if cursorPosition.y == 24 {
                        scrollUp();
                    } else {
                        cursorPosition.y += 1;
                    }
                    cursorPosition.x = 0;
                }
                let currentOffset = calculatedOffset(&cursorPosition);

                *vgaBuffer.offset(currentOffset) = byte;

                // On real hardware, the VGA mode we're in only allows 3 bits for the background, 4bit is blinking :|
                // BUGUBG: Figure out how to switch modes so we can use the 4th bit for more background colors
                // https://old.reddit.com/r/osdev/comments/70fcig/blinking_text/
                *vgaBuffer.offset(currentOffset + 1) =
                    getColorByte(ForegroundColor::Green, BackgroundColor::Black);

                cursorPosition.x += 1;
            }
        }
    }

    setCursorPosition(&cursorPosition);
}

fn calculatedOffset(cursorPosition: &CursorPosition) -> isize {
    let mut result = cursorPosition.y as u16;
    result *= VGA_WIDTH;
    result += cursorPosition.x as u16;
    result *= 2; // Each character takes up 2 bytes in the buffer

    return result as isize;
}

fn getCursorPosition() -> CursorPosition {
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

fn setCursorPosition(pos: &CursorPosition) {
    let mut positionOffset: u16 = pos.y as u16;
    positionOffset *= VGA_WIDTH;
    positionOffset += pos.x as u16;

    unsafe {
        outB(VGA_ADDRESS_PORT, CURSOR_HIGH_REG);
        outB(VGA_DATA_PORT, (positionOffset >> 8) as u8);

        outB(VGA_ADDRESS_PORT, CURSOR_LOW_REG);
        outB(VGA_DATA_PORT, positionOffset as u8);
    }
}
