use once_cell::sync::Lazy;

use crate::vgaWriteLine;
use crate::serial::serialPort::{COMPort, SerialPort};

pub static SYSTEM_LOGGER : Lazy<Logger> = Lazy::new(|| Logger::new());

pub struct Logger {
    serial: Option<SerialPort>,
    initAttempted: bool,
    initSuccess: bool,
}

impl Logger {
    const fn new() -> Self {
        Logger {
            serial: None,
            initAttempted: false,
            initSuccess: false,
        }
    }

    pub fn Write(&self, msg: &[u8]) {
        if self.serial.is_some() {
            _ = self.serial.as_ref().unwrap().Send(msg);
        }

        crate::textMode::vga::writeString(msg);
    }
}
