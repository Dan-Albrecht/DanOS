use core::fmt::{Result, Write};

use super::logger::SYSTEM_LOGGER;

pub struct LogWriter;

impl LogWriter {
    pub fn new() -> Self {
        LogWriter
    }
}

impl Write for LogWriter {
    fn write_str(&mut self, s: &str) -> Result {
        SYSTEM_LOGGER.Write(s.as_bytes());
        Ok(())
    }
}
