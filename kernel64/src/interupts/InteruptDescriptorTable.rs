use core::arch::asm;
use core::mem::size_of;
use core::ptr::addr_of;

use crate::assemblyHelpers::breakpoint::HaltLoop;
use crate::vgaWriteLine;

use super::setup::SetupStuff;
use core::fmt::Write;

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
#[repr(C, align(8))]
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

#[inline(never)]
#[no_mangle]
pub fn InterruptHandlerIntImpl(vector: u8, stackFrame: ExceptionStackFrame) {
    let cs = stackFrame.CodeSegment;
    let ip = stackFrame.InstructionPointer;
    let flags = stackFrame.CpuFlags;
    let ss = stackFrame.StackSegment;
    let sp = stackFrame.StackPointer;
    vgaWriteLine!(
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
        HaltLoop();
    }
}

#[inline(never)]
#[no_mangle]
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
    vgaWriteLine!(
        "Ex 0x{:X} with code 0x{:X}. CS: 0x{:X} IP: 0x{:X} Flags: 0x{:X} SS: 0x{:X} SP: 0x{:X}",
        vector,
        errorCode,
        cs,
        ip,
        flags,
        ss,
        sp,
    );

    HaltLoop();
}

#[inline(never)] // BUGBUG: Just temp for debugging
pub fn SetIDT() {
    let randomPointer = 0x12000 as *mut Table;
    SetupStuff(randomPointer);

    let idtr = IDTR {
        Base: randomPointer as usize,
        Limit: size_of::<Table>() as u16 * 256 - 1, // BUGBUG: Don't hardcode
    };

    unsafe {
        asm!(
            "lidt [{0}]",
            "sti",
            in(reg) addr_of!(idtr),
        );
    }
}

pub fn DisableInterrupts() {
    unsafe {
        asm!("cli");
    }
}
