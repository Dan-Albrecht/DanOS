use core::{cell::{self, UnsafeCell}, fmt::Write};
use kernel_shared::vgaWriteLine;
use critical_section::RawRestoreState;
use once_cell::sync::Lazy;

use crate::serial::serialPort::{COMPort, SerialPort};

struct DanOSCriticalSection;
critical_section::set_impl!(DanOSCriticalSection);

unsafe impl critical_section::Impl for DanOSCriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        // We're currently running with interrupts disabled so nothing to do right now
    }

    unsafe fn release(_restore_state: RawRestoreState) {
    }
}

pub static SystemLogger : Lazy<Logger> = Lazy::new(|| Logger::new());

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
            use core::fmt::Write;
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
        let serial = SerialPort::tryGet(COMPort::COM1);
        if serial.is_none() {
            vgaWriteLine!("Failed to init serial port...");
        }

        Logger { serial: serial }
    }

    pub fn Write(&self, msg: &[u8]) {
        // BUGBUG: This almost certianly means I'm doing it wrong...
        // This was a 'just make it compile' thing
        if self.serial.is_some() {
            let _ = self.serial.as_ref().unwrap().Send(msg);
        }

        kernel_shared::textMode::vga::writeString(msg);
    }
}
