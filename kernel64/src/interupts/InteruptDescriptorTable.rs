use core::arch::asm;
use core::mem::size_of;

use kernel_shared::assemblyStuff::halt::haltLoop;
use kernel_shared::memoryHelpers::zeroMemory2;
use kernel_shared::physicalMemory::PhysicalMemoryManager;

use crate::assemblyHelpers::getCR2;
use crate::loggerWriteLine;

use crate::memory::memoryStuff::MemoryStuff;

use super::setup::SetupStuff;

// See Intel Volume 3A, Chapter 6: Interrupt and Exception Handling
#[repr(C, packed)]
pub struct Entry {
    pub IsrLow: u16,    // Bits 0..=15 of ISR address
    pub CS: u16,        // Code segment (CS register) that'll be set to get to the ISR
    pub _IST: u8,       // Interup Stack Table. Don't plan to use currently.
    pub Attributes: u8, // See usage
    pub IsrMid: u16,    // Bits 16..=31 of ISR address
    pub IsrHigh: u32,   // Bits 32..=63 of ISR address
    pub Zero: u32,      // Reserved
}

impl Default for Entry {
    fn default() -> Self {
        Entry {
            IsrLow: 0,
            CS: 0,
            _IST: 0,
            Attributes: 0,
            IsrMid: 0,
            IsrHigh: 0,
            Zero: 0,
        }
    }
}

// https://internals.rust-lang.org/t/conflation-of-alignment-and-packing/10443
#[repr(C, align(16))]
pub struct Table {
    pub Table: Table_,
}

#[repr(C, packed)]
pub struct Table_ {
    pub Entries: [Entry; 256],
}

#[repr(C, packed)]
pub struct IDTR {
    Limit: u16,
    Base: usize,
}

#[repr(C, packed)]
pub struct ExceptionStackFrame {
    InstructionPointer: usize,
    CodeSegment: usize,
    CpuFlags: usize,
    StackPointer: usize,
    StackSegment: usize,
}

// Interupt Descriptor Table
pub struct IDT {
    idtr: *mut IDTR,
}

impl IDT {
    pub fn new(mem: &mut impl MemoryStuff) -> Self {
        let idt: *mut Table = mem.allocate();

        unsafe {
            zeroMemory2(idt);
        }

        SetupStuff(idt);
        let limit: u16;
        let entrySize = size_of::<Entry>();

        unsafe {
            let length = (*idt).Table.Entries.len();

            // The last byte of the table
            limit = (entrySize * length - 1) as u16;
            loggerWriteLine!(
                "IDT @ 0x{:X}. Entry Size: 0x{:X} Length: 0x{:X}. Limit: 0x{:X}.",
                idt as usize,
                entrySize,
                length,
                limit
            );
        }

        let idtr: *mut IDTR = mem.allocate();

        unsafe {
            (*idtr).Base = idt as usize;
            (*idtr).Limit = limit;

            asm!(
                "lidt [{0}]",
                //"ljmp $2f", // BUGBUG? OS Dev says do a long jump after loading the table
                "2:",
                "sti",
                in(reg) idtr,
            );
        }

        return IDT { idtr };
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub fn InterruptHandlerIntImpl(vector: u8, stackFrame: ExceptionStackFrame) {
    let cs = stackFrame.CodeSegment;
    let ip = stackFrame.InstructionPointer;
    let flags = stackFrame.CpuFlags;
    let ss = stackFrame.StackSegment;
    let sp = stackFrame.StackPointer;
    loggerWriteLine!(
        "Interrupt 0x{:X}. CS: 0x{:X} IP: 0x{:X} Flags: 0x{:X} SS: 0x{:X} SP: 0x{:X}",
        vector,
        cs,
        ip,
        flags,
        ss,
        sp,
    );

    // Breakpoint can resume
    if vector != 3 {
        haltLoop();
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub fn InterruptHandlerWithCodeIntImpl(
    vector: u8,
    stackFrame: ExceptionStackFrame,
    errorCode: u64,
) {
    let cs = stackFrame.CodeSegment;
    let ip = stackFrame.InstructionPointer;
    let flags = stackFrame.CpuFlags;
    let ss = stackFrame.StackSegment;
    let sp = stackFrame.StackPointer;
    loggerWriteLine!(
        "Ex 0x{:X} with code 0x{:X}. CS: 0x{:X} IP: 0x{:X} Flags: 0x{:X} SS: 0x{:X} SP: 0x{:X}",
        vector,
        errorCode,
        cs,
        ip,
        flags,
        ss,
        sp,
    );

    if vector == 0xE {
        let address = getCR2();
        loggerWriteLine!("Page Fault at virtual address: 0x{:X}", address);
    }

    haltLoop();
}

pub unsafe fn SetIDT(memoryManager: &mut PhysicalMemoryManager) -> usize {
    let idt: *mut Table = memoryManager.ReserveWhereverZeroed2();

    loggerWriteLine!("IDT @ 0x{:X}", idt as usize);

    SetupStuff(idt);
    let limit: u16;

    unsafe {
        let size = size_of::<Entry>();
        let length = (*idt).Table.Entries.len();

        // The last byte of the table
        limit = (size * length - 1) as u16;
        loggerWriteLine!(
            "IDT @ 0x{:X}. Entry Size: 0x{:X} Length: 0x{:X}. Limit: 0x{:X}.",
            idt as usize,
            size,
            length,
            limit
        );
    }

    let idtr : *mut IDTR = memoryManager.ReserveWhereverZeroed2();
    unsafe {
        (*idtr).Base = idt as usize;
        (*idtr).Limit = limit;
    }

    loggerWriteLine!("IDTR @ 0x{:X}", idtr as usize);
    unsafe {
        asm!(
            "lidt [{0}]",
            //"ljmp $2f", // BUGBUG? OS Dev says do a long jump after loading the table
            "2:",
            "sti",
            in(reg) idtr,
        );
    }

    return idt as usize;
}
