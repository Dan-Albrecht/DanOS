use kernel_shared::{assemblyStuff::ports::inB, loggerWrite, loggerWriteLine};

use crate::memory::virtualMemory::VirtualMemoryManager;

pub struct KernelShell<'a> {
    stuff: &'a VirtualMemoryManager,
}

// http://www.brokenthorn.com/Resources/OSDev19.html
const STATUS_REGISTER_PORT: u16 = 0x64;
const INPUT_BUFFER_PORT: u16 = 0x60;

struct StatusRegister {
    out_buffer_full: bool,
    in_buffer_full: bool,
}

impl<'a> KernelShell<'a> {
    pub fn new(vmm: &'a mut VirtualMemoryManager) -> Self {
        KernelShell { stuff: vmm }
    }

    pub fn run(&self) {
        loggerWriteLine!("Kernel shell is running...");
        loggerWrite!("> ");

        loop {
            unsafe {
                let status = inB(STATUS_REGISTER_PORT);
                let sr = StatusRegister {
                    out_buffer_full: status & 0x01 != 0,
                    in_buffer_full: status & 0x02 != 0,
                };

                if sr.out_buffer_full == false {
                    continue;
                }

                let input = inB(INPUT_BUFFER_PORT);
                if let Some(c) = self.translate_potential_scan_code(input) {
                    loggerWrite!("{}", c);
                }
            }
        }
    }

    fn translate_potential_scan_code(&self, scancode: u8) -> Option<char> {
        match scancode {
            0x9E => Some('A'),
            0xB0 => Some('B'),
            0xAE => Some('C'),
            0xA0 => Some('D'),
            0x92 => Some('E'),
            0xA1 => Some('F'),
            0xA2 => Some('G'),
            0xA3 => Some('H'),
            0x97 => Some('I'),
            0xA4 => Some('J'),
            0xA5 => Some('K'),
            0xA6 => Some('L'),
            0xB2 => Some('M'),
            0xB1 => Some('N'),
            0x98 => Some('O'),
            0x99 => Some('P'),
            0x90 => Some('Q'),
            0x93 => Some('R'),
            0x9F => Some('S'),
            0x94 => Some('T'),
            0x96 => Some('U'),
            0xAF => Some('V'),
            0x91 => Some('W'),
            0xAD => Some('X'),
            0x95 => Some('Y'),
            0xAC => Some('Z'),

            0x8B => Some('0'),
            0x82 => Some('1'),
            0x83 => Some('2'),
            0x84 => Some('3'),
            0x85 => Some('4'),
            0x86 => Some('5'),
            0x87 => Some('6'),
            0x88 => Some('7'),
            0x89 => Some('8'),
            0x8A => Some('9'),

            0xB9 => Some(' '),

            // Punctuation (no shift)
            0x8C => Some('-'),
            0x8D => Some('='),
            0x9A => Some('['),
            0x9B => Some(']'),
            0xAB => Some('\\'),
            0xA7 => Some(';'),
            0xA8 => Some('\''),
            0xA9 => Some('`'),
            0xB3 => Some(','),
            0xB4 => Some('.'),
            0xB5 => Some('/'),

            // Numpad numbers (assuming NumLock is on)
            0xD2 => Some('0'),
            0xCF => Some('1'),
            0xD0 => Some('2'),
            0xD1 => Some('3'),
            0xCB => Some('4'),
            0xCC => Some('5'),
            0xCD => Some('6'),
            0xC7 => Some('7'),
            0xC8 => Some('8'),
            0xC9 => Some('9'),
            0xD3 => Some('.'),
            // Forard slash is same as above (BUGBUG: Because we currenly ignore E0 modifier)
            0xB7 => Some('*'),
            0xCA => Some('-'),
            0xCE => Some('+'),
            
            _ => None,
        }
    }
}
