use core::fmt::Write;

use super::textMode::writeString;

pub struct Writer;

impl Writer {
    pub fn new() -> Self {
        Writer
    }
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        writeString(s.as_bytes());
        Ok(())
    }
}
