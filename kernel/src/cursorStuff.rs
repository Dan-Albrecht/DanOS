const VGA_WIDTH: u16 = 80;
const VGA_HEIGHT: u16 = 25;

const VGA_ADDRESS_PORT: u16 = 0x3D4;
const VGA_DATA_PORT: u16 = 0x3D5;
const CURSOR_HIGH_REG: u8 = 0xE;
const CURSOR_LOW_REG: u8 = 0xF;
const VGA_BUFFER_ADDRESS: u32 = 0xB8000;

use crate::assemblyStuff::ports::{inB, outB};

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

pub unsafe fn writeStringOnNewline(msg: &'static [u8]) {
    let vgaBuffer = VGA_BUFFER_ADDRESS as *mut u8;
    let mut bufferOffset: u16;
    let mut cursorPosition: CursorPosition;

    unsafe {
        cursorPosition = getCursorPosition();
        cursorPosition.x = 0;

        if cursorPosition.y == 24 {
            scrollUp();
        } else {
            cursorPosition.y += 1;
        }

        bufferOffset = cursorPosition.y as u16;
        bufferOffset *= VGA_WIDTH;
        bufferOffset += cursorPosition.x as u16;
        bufferOffset *= 2; // Each character takes up 2 bytes in the buffer
    }

    for (i, &byte) in msg.iter().enumerate() {
        unsafe {
            let mut currentOffset = bufferOffset as isize;
            currentOffset += (i * 2) as isize;
            *vgaBuffer.offset(currentOffset) = byte;
            *vgaBuffer.offset(currentOffset + 1) = 0x74; // Red on gray

            cursorPosition.x += 1;
        }
    }

    setCursorPosition(&cursorPosition);
}

pub unsafe fn getCursorPosition() -> CursorPosition {
    outB(VGA_ADDRESS_PORT, CURSOR_HIGH_REG);
    let mut position = inB(VGA_DATA_PORT) as u16;
    position <<= 8; // Move to high byte.

    outB(VGA_ADDRESS_PORT, CURSOR_LOW_REG);
    position |= inB(VGA_DATA_PORT) as u16;

    let x = (position % VGA_WIDTH) as u8;
    let y = (position / VGA_WIDTH) as u8;

    CursorPosition { x, y }
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
