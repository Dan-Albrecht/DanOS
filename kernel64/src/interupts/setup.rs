use super::table::Interrupt0;
use super::table::Interrupt3;
use super::table::Interrupt14;
//use crate::vgaWriteLine;
use super::InteruptDescriptorTable::{Entry, Table};
//use core::fmt::Write;

pub fn SetupStuff(table: *mut Table) {
    unsafe {
        SetAddress(&mut (*table).Table.Entries[0], Interrupt0 as u64, 0);
        SetAddress(&mut (*table).Table.Entries[3], Interrupt3 as u64, 3);
        SetAddress(&mut (*table).Table.Entries[14], Interrupt14 as u64, 14);
    }
}

#[inline(never)]
#[no_mangle]
fn SetAddress(entry: &mut Entry, address: u64, _index: u16) {
    //vgaWriteLine!("Setting interrupt 0x{:X} to 0x{:X}", index, address);
    entry.IsrHigh = (address >> 32) as u32;
    entry.IsrMid = ((address >> 16) & 0xFFFF) as u16;
    entry.IsrLow = (address & 0xFFFF) as u16;
    entry.CS = 8;

    entry.Attributes = 0x8E; // BUGBUG: Varry by type
    entry.Zero = 0;
    entry._IST = 0;
}
