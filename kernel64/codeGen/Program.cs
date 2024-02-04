using System;
using System.IO;
using System.Linq;

const string tableProlog = """
use super::InteruptDescriptorTable::{ExceptionStackFrame, InterruptHandlerIntImpl, InterruptHandlerWithCodeIntImpl};
""";

const string noCodeStub = """
#[inline(never)]
#[no_mangle]
pub extern "x86-interrupt" fn Interrupt__SUFFIX__(stackFrame: ExceptionStackFrame) {
    InterruptHandlerIntImpl(__SUFFIX__, stackFrame);
}
""";

const string codeStub = """
#[inline(never)]
#[no_mangle]
pub extern "x86-interrupt" fn Interrupt__SUFFIX__(stackFrame: ExceptionStackFrame, errorCode: u64) {
    InterruptHandlerWithCodeIntImpl(__SUFFIX__, stackFrame, errorCode);
}

""";

const string setupProlog = """
use crate::vgaWriteLine;
use super::InteruptDescriptorTable::{Entry, Table};
use core::fmt::Write;

pub fn SetupStuff(table: *mut Table) {
    unsafe {

""";

const string setupRegistration = """
        SetAddress(&mut (*table).Table.Entries[__SUFFIX__], Interrupt__SUFFIX__ as u64, __SUFFIX__);
""";

const string setupUse = """
use super::table::Interrupt__SUFFIX__;

""";

const string setupEpilog = """
    }
}

#[inline(never)]
#[no_mangle]
fn SetAddress(entry: &mut Entry, address: u64, index: u16) {
    vgaWriteLine!("Setting interrupt 0x{:X} to 0x{:X}", index, address);
    entry.IsrHigh = (address >> 32) as u32;
    entry.IsrMid = ((address >> 16) & 0xFFFF) as u16;
    entry.IsrLow = (address & 0xFFFF) as u16;
    entry.CS = 8;

    entry.Attributes = 0x8E; // BUGBUG: Varry by type
    entry.Zero = 0;
    entry._IST = 0;
}
""";

var exceptionsToHandle = new int[] { 0, 3, 14 };

// Rust macros are too limited for simply identifer concatentation
// So just do this in C#. Should probably just do this in a standalone Rust program though...
using var tableStream = new FileStream("table.rs", FileMode.Create);
using var tableWriter = new StreamWriter(tableStream);
using var setupStream = new FileStream("setup.rs", FileMode.Create);
using var setupWriter = new StreamWriter(setupStream);

for (int i = 0; i <= 255; i++)
{
    if (!exceptionsToHandle.Contains(i))
    {
        continue;
    }

    var setup = setupUse.Replace("__SUFFIX__", i.ToString());
    setupWriter.Write(setup);
}

setupWriter.Write(setupProlog);

tableWriter.Write(tableProlog);
tableWriter.Write("\n\n");

for (int i = 0; i <= 255; i++)
{
    if (!exceptionsToHandle.Contains(i))
    {
        continue;
    }

    string stub;

    if (i == 8 || (i >= 10 && i <= 14) || i == 17 || i == 30)
    {
        stub = codeStub;
    }
    else
    {
        stub = noCodeStub;
    }

    stub = stub.Replace("__SUFFIX__", i.ToString());
    tableWriter.Write(stub);
    tableWriter.Write("\n");
    tableWriter.Write("\n");

    var setup = setupRegistration.Replace("__SUFFIX__", i.ToString());
    setupWriter.Write(setup);
    setupWriter.Write("\n");
}
setupWriter.Write(setupEpilog);
setupWriter.Write("\n");
