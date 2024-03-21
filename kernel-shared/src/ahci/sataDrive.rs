use crate::{
    assemblyStuff::halt::haltLoop,
    magicConstants::{
        SATA_DRIVE_BASE_CMD_BASE_ADDRESS, SATA_DRIVE_BASE_COMMAND_TABLE_BASE_ADDRESS,
        SATA_DRIVE_BASE_FIS_BASE_ADDRESS,
    },
    vgaWriteLine,
};

use super::controller::Controller;
use core::{
    fmt::Write,
    mem::{offset_of, size_of},
    ptr::{addr_of, read_volatile},
};

pub struct SataDrive {
    Controller: Controller,
    Port: u8,
}
impl SataDrive {
    pub fn stopCommands(&self) {
        let port = self.Controller.getPort(self.Port);
        unsafe {
            (*port).CMD &= !CMD_START_MASK;
            (*port).CMD &= !CMD_FRE_MASK;

            let mask = CMD_FR_MASK | CMD_CR_MASK;

            vgaWriteLine!("Stopping commands...");

            loop {
                let value = read_volatile(addr_of!((*port).CMD));
                if value & mask == 0 {
                    break;
                }
            }

            vgaWriteLine!("Commands stopped");
        }
    }

    pub fn startCommands(&self) {
        let port = self.Controller.getPort(self.Port);
        unsafe {
            vgaWriteLine!("Waiting to start...");

            loop {
                let value = read_volatile(addr_of!((*port).CMD));
                if value & CMD_CR_MASK == 0 {
                    break;
                } 
            }

            (*port).CMD |= CMD_FRE_MASK;
            (*port).CMD |= CMD_START_MASK;

            vgaWriteLine!("Commands started");
        }
    }

    pub(crate) fn new(controller: Controller, port: u8) -> SataDrive {
        SataDrive {
            Controller: controller,
            Port: port,
        }
    }

    // Port (0 .. 32)
    //   CommandList
    //     CommandHeader (0 .. 32)
    //       CommandTable
    //         PRDT (0 .. COUNT_OF_PRDT)
    // BUGBUG: This all breaks if we attempt to use this for multiple ports, we assume only one
    pub fn remapStuff(&self) {
        let port = self.Controller.getPort(self.Port);
        unsafe {
            (*port).CLB = SATA_DRIVE_BASE_CMD_BASE_ADDRESS;

            // In 32-bit space
            (*port).CLBU = 0;

            let bytePointer = (*port).CLB as *mut u8;

            for offset in 0..size_of::<CommandList>() {
                *(bytePointer.offset(offset as isize)) = 0;
            }

            (*port).FB = SATA_DRIVE_BASE_FIS_BASE_ADDRESS;

            // In 32-bit space
            (*port).FBU = 0;

            let bytePointer = (*port).FB as *mut u8;

            // BUGBUG: Is this the right ammount?
            // BUGBUG: Create a 0-memory function
            for offset in 0..256 {
                *(bytePointer.offset(offset)) = 0;
            }

            let cl = (*port).CLB as *mut CommandList;
            for index in 0..32 {
                let header = (*cl).getHeader(index);

                (*header).setPrdtl(COUNT_OF_PRDT);

                let offset = CommandTable::getFullLength() * index as usize;
                (*header).setCommandTable(SATA_DRIVE_BASE_COMMAND_TABLE_BASE_ADDRESS + offset)
            }
        }
    }
}

impl CommandTable {
    pub fn getFullLength() -> usize {
        // -1 as the table defintion already alocates the first entry
        return size_of::<CommandTable>() + size_of::<PRDT>() * ((COUNT_OF_PRDT - 1) as usize);
    }
}

impl CommandList {
    pub fn getHeader(&self, number: u8) -> *mut CommandHeader {
        let header = match number {
            0 => &self.C0,
            1 => &self.C1,
            2 => &self.C2,
            3 => &self.C3,
            4 => &self.C4,
            5 => &self.C5,
            6 => &self.C6,
            7 => &self.C7,
            8 => &self.C8,
            9 => &self.C9,
            10 => &self.C10,
            11 => &self.C11,
            12 => &self.C12,
            13 => &self.C13,
            14 => &self.C14,
            15 => &self.C15,
            16 => &self.C16,
            17 => &self.C17,
            18 => &self.C18,
            19 => &self.C19,
            20 => &self.C20,
            21 => &self.C21,
            22 => &self.C22,
            23 => &self.C23,
            24 => &self.C24,
            25 => &self.C25,
            26 => &self.C26,
            27 => &self.C27,
            28 => &self.C28,
            29 => &self.C29,
            30 => &self.C30,
            31 => &self.C31,
            _ => {
                vgaWriteLine!("{number} is a bogus command header");
                haltLoop();
            }
        };

        // BUGBUG: Casting mutable again...
        return header as *const _ as *mut CommandHeader;
    }
}

impl CommandHeader {
    // Physical Region Descriptor Table Length
    pub fn setPrdtl(&mut self, value: u16) {
        let mut value = value as u32;
        value <<= 16;

        self.DW0 &= 0xFFFF;
        self.DW0 |= value;
    }

    pub fn setCommandTable(&mut self, address: usize) {
        if address & 0x7F != 0 {
            vgaWriteLine!("{address} is not properly aligned");
            haltLoop();
        }

        if address > u32::MAX as usize {
            vgaWriteLine!("Address it out of range");
            haltLoop();
        }

        // Command Table Descriptor Base Address (CTBA)
        self.DW2 = address as u32;

        // We're only doing 32 bit addresses for now
        // Command Table Descriptor Base Address Upper 32-bits (CTBAU)
        self.DW3 = 0;
    }
}

// 3.3.7 Offset 18h: PxCMD – Port x Command and Status

// Start (ST)
// RW
const CMD_START_MASK: u32 = 1 << 0;

// FIS Receive Enable (FRE)
// RW
const CMD_FRE_MASK: u32 = 1 << 4;

// FIS Receive Running (FR)
// RO
const CMD_FR_MASK: u32 = 1 << 14;

// Command List Running (CR)
// RO
const CMD_CR_MASK: u32 = 1 << 15;

// Physical Region Descriptor Table
// BUGBUG: This is the value OSDev went with, should figure out why...
const COUNT_OF_PRDT: u16 = 8;

// 4.2.3 Command Table
#[repr(C, packed)]
pub struct CommandTable {
    // 4.2.3.1 Command FIS (CFIS)
    CFIS: [u8; 64],
    // 4.2.3.2 ATAPI Command (ACMD)
    ACMD: [u8; 16],
    Reserved: [u8; 0x30],
    // 4.2.3.3 Physical Region Descriptor Table (PRDT)
    PRDT0: PRDT,
    // PRTDX follows
}

// 4.2.3.3 Physical Region Descriptor Table (PRDT)
#[repr(C, packed)]
pub struct PRDT {
    DW0: u32,
    DW1: u32,
    DW2: u32,
    DW3: u32,
}

// 4.2.2 Command List Structure
#[repr(C, packed)]
pub struct CommandList {
    C0: CommandHeader,
    C1: CommandHeader,
    C2: CommandHeader,
    C3: CommandHeader,
    C4: CommandHeader,
    C5: CommandHeader,
    C6: CommandHeader,
    C7: CommandHeader,
    C8: CommandHeader,
    C9: CommandHeader,
    C10: CommandHeader,
    C11: CommandHeader,
    C12: CommandHeader,
    C13: CommandHeader,
    C14: CommandHeader,
    C15: CommandHeader,
    C16: CommandHeader,
    C17: CommandHeader,
    C18: CommandHeader,
    C19: CommandHeader,
    C20: CommandHeader,
    C21: CommandHeader,
    C22: CommandHeader,
    C23: CommandHeader,
    C24: CommandHeader,
    C25: CommandHeader,
    C26: CommandHeader,
    C27: CommandHeader,
    C28: CommandHeader,
    C29: CommandHeader,
    C30: CommandHeader,
    C31: CommandHeader,
}

// 4.2.2 Command List Structure
#[repr(C, packed)]
pub struct CommandHeader {
    DW0: u32,
    DW1: u32,
    DW2: u32,
    DW3: u32,
    DW4: u32,
    DW5: u32,
    DW6: u32,
    DW7: u32,
}
