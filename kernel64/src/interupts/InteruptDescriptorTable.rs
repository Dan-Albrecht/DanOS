use core::arch::asm;
use core::ptr::addr_of;
use core::mem::size_of;
use core::fmt::Write;
use crate::vgaWriteLine;

// See Intel Volume 3A, Chapter 6: Interrupt and Exception Handling
#[repr(C, packed)]
pub struct Entry {
    IsrLow: u16,    // Bits 0..=15 of ISR address
    CS: u16,        // Code segment (CS register) that'll be set to get to the ISR
    _IST: u8,       // Interup Stack Table. Don't plan to use currently.
    Attributes: u8, // See usage
    IsrMid: u16,    // Bits 16..=31 of ISR address
    IsrHigh: u32,   // Bits 32..=63 of ISR address
    Zero: u32,      // Reserved
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
    Table: Table_,
}

#[repr(C, packed)]
pub struct Table_ {
    Entries: [Entry; 256],
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

// BUGBUG: Best gyrations I can come up with to keep the actual interrup handler from getting stripped/optimized away
#[used(linker)]
static DONT_STRIP_THIS: extern "x86-interrupt" fn(ExceptionStackFrame) = InterruptHandler;

#[used(linker)]
static DONT_STRIP_THIS2: extern "x86-interrupt" fn(ExceptionStackFrame, u64) =
    InterruptHandlerWithCode;

#[inline(never)]
pub extern "x86-interrupt" fn InterruptHandler(stackFrame: ExceptionStackFrame) {
    let cs = stackFrame.CodeSegment;
    let ip = stackFrame.InstructionPointer;
    vgaWriteLine!("Interupt without error code. CS: 0x{:X} IP: 0x{:X}", cs, ip);
}

#[inline(never)]
pub extern "x86-interrupt" fn InterruptHandlerWithCode(
    stackFrame: ExceptionStackFrame,
    errorCode: u64,
) {
    let cs = stackFrame.CodeSegment;
    let ip = stackFrame.InstructionPointer;
    vgaWriteLine!("Interupt with error code 0x{:X}. CS: 0x{:X} IP: 0x{:X}", errorCode, cs, ip);
}

#[inline(never)] // BUGBUG: Just temp for debugging
pub fn SetIDT() {

    let randomPointer = 0x10000 as *mut Table;
    let func = InterruptHandler;
    let funAddress = func as u64;
    let otherFunc = InterruptHandlerWithCode;
    let otherAddress = otherFunc as u64;
    let length:u16;

    unsafe{
        length = (*randomPointer).Table.Entries.len() as u16;
        for index in 0..(*randomPointer).Table.Entries.len() {

            if index == 8 || (index >= 10 && index <= 14) || index == 17 ||
            index == 30 {
                SetAddress(&mut (*randomPointer).Table.Entries[index], otherAddress);
            } else {
                SetAddress(&mut (*randomPointer).Table.Entries[index], funAddress);
            }

            (*randomPointer).Table.Entries[index].Attributes = 0x8E; // BUGBUG: Varry by type
            (*randomPointer).Table.Entries[index].CS = 0x8;
            (*randomPointer).Table.Entries[index].Zero = 0;
            (*randomPointer).Table.Entries[index]._IST=0;
        }
    
    }

    let idtr = IDTR{
        Base: randomPointer as usize,
        Limit: size_of::<Table>() as u16 * length - 1,
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

fn SetAddress(entry: &mut Entry, address: u64) {
    entry.IsrHigh = (address >> 32) as u32;
    entry.IsrMid = ((address >> 16) & 0xFFFF) as u16;
    entry.IsrLow = (address & 0xFFFF) as u16;
    entry.CS = 8;
}
