use core::fmt::Write;
use kernel_shared::vgaWriteLine;
use lazy_static::lazy_static;

use crate::serial::serialPort::{COMPort, SerialPort};

lazy_static! {
    pub static ref SystemLogger: Logger = Logger::new();
}

pub struct Logger {
    serial: Option<SerialPort>,
}

pub struct LogWriter;

impl LogWriter {
    pub fn new() -> Self {
        LogWriter
    }
}

impl Write for LogWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        SystemLogger.Write(s.as_bytes());
        Ok(())
    }
}

#[macro_export]
macro_rules! loggerWrite {
    ($($args:tt)*) => {
        if let Some(formattedString) = core::format_args!($($args)*).as_str() {
            $crate::logging::logger::SystemLogger.Write(formattedString.as_bytes());
        } else {
            let _ = write!($crate::logging::logger::LogWriter::new(), $($args)*);
        }
    };
}

#[macro_export]
macro_rules! loggerWriteLine {
    ($($args:tt)*) => {
        $crate::loggerWrite!($($args)*);
        $crate::logging::logger::SystemLogger.Write(b"\r\n");
    };
}

impl Logger {
    fn new() -> Self {
        // BUGBUG: This will currently hang on real hard, need to fix the init code to not loop forever
        //let serial = SerialPort::tryGet(COMPort::COM1);
        let serial = None;
        if serial.is_none() {
            vgaWriteLine!("Failed to init serial port...");
        }

        Logger { serial: serial }
    }

    pub fn Write(&self, msg: &[u8]) {
        // BUGBUG: This almost certianly means I'm doing it wrong...
        // This was a 'just make it compile' thing
        if self.serial.is_some() {
            self.serial.as_ref().unwrap().Send(msg);
        }

        kernel_shared::textMode::textMode::writeString(msg);
    }
}
