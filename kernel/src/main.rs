#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![feature(asm_const)]

use core::{arch::asm, panic::PanicInfo};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static WELCOME_MSG: &[u8] = b"We've made it to Rust!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vgaBuffer = 0xB8000 as *mut u8;
    let mut bufferOffset: u16;
    let mut cursorPosition: CursorPosition;

    unsafe {
        cursorPosition = getCursorPosition();
        cursorPosition.x = 0;
        cursorPosition.y += 1;
        bufferOffset = cursorPosition.y as u16;
        bufferOffset *= VGA_WIDTH;
        bufferOffset += cursorPosition.x as u16;
        bufferOffset *= 2; // Each character takes up 2 bytes in the buffer
        setCursorPosition(&cursorPosition);
    }

    for (i, &byte) in WELCOME_MSG.iter().enumerate() {
        unsafe {
            let mut currentOffset = bufferOffset as isize;
            currentOffset += (i * 2) as isize;
            *vgaBuffer.offset(currentOffset) = byte;
            *vgaBuffer.offset(currentOffset + 1) = 0x74; // Red on gray

            // BUGBUG: Should probably just set this once at the end
            cursorPosition.x += 1;
            setCursorPosition(&cursorPosition);
        }
    }

    loop {}
}

pub struct CursorPosition {
    x: u8,
    y: u8,
}

const VGA_ADDRESS_PORT: u16 = 0x3D4;
const VGA_DATA_PORT: u16 = 0x3D5;
const CURSOR_HIGH_REG: u8 = 0xE;
const CURSOR_LOW_REG: u8 = 0xF;
const VGA_WIDTH: u16 = 80;

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

pub unsafe fn outB(port: u16, value: u8) {
    asm!(
        "out dx, al",
        in("dx") port,
        in("al") value
    );
}

pub unsafe fn inB(port: u16) -> u8 {
    let result: u8;
    asm!(
        "in al, dx",
        out("al") result,
        in("dx") port
    );

    result
}
