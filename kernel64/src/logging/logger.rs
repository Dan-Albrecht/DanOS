use kernel_shared::vgaWriteLine;
use once_cell::sync::Lazy;

use crate::serial::serialPort::{COMPort, SerialPort};

pub static SYSTEM_LOGGER : Lazy<Logger> = Lazy::new(|| Logger::new());

pub struct Logger {
    serial: Option<SerialPort>,
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
