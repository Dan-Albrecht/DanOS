use core::fmt::{self, Write};

use super::vga::{self};

pub struct Writer;

impl Writer {
    pub fn new() -> Self {
        Writer
    }
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        vga::writeString(s.as_bytes());
        Ok(())
    }
}
