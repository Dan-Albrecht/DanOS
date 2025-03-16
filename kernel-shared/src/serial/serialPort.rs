use crate::{
    assemblyStuff::ports::{inB, outB},
    vgaWriteLine,
};

// Do not use loggerWrite* functions in here as it may not be properly setup

pub struct SerialPort {
    port: COMPort,
}

#[derive(Debug)]
pub enum COMPort {
    COM1,
}

#[derive(Debug)]
pub enum SerialFailure {
    Timeout,
}

#[cfg(target_pointer_width = "32")]
const MAX_LOOP_VALUE: usize = 0xFFFF_FFFF;

#[cfg(target_pointer_width = "64")]
const MAX_LOOP_VALUE: usize = 0xFFFF_FFFF_FFFF;

// https://wiki.osdev.org/Serial_Ports
impl SerialPort {
    pub fn tryGet(port: COMPort) -> Option<SerialPort> {
        let result = SerialPort { port: port };

        unsafe {
            let initResult = result.init();
            if initResult.is_err() {
                vgaWriteLine!(
                    "Failed to init serial with {:?}",
                    initResult.as_ref().unwrap_err()
                );
                return None;
            }

            if initResult.unwrap() {
                Some(result)
            } else {
                None
            }
        }
    }

    pub fn Send(&self, msg: &[u8]) -> Result<(), SerialFailure> {
        for index in 0..msg.len() {
            unsafe {
                self.sendByte(msg[index])?;
            }
        }

        Ok(())
    }

    unsafe fn sendByte(&self, b: u8) -> Result<(), SerialFailure> {
        let mut x = 0;
        while self.isTransmitNotEmpty() {
            x += 1;
            if x == MAX_LOOP_VALUE {
                return Err(SerialFailure::Timeout);
            }
        }

        outB(self.port.getPortAddress() + 0, b);
        Ok(())
    }

    unsafe fn receiveByte(&self) -> Result<u8, SerialFailure> {
        let mut x = 0;
        while !self.dataAvailable() {
            x += 1;
            if x == MAX_LOOP_VALUE {
                return Err(SerialFailure::Timeout);
            }
        }

        let b = inB(self.port.getPortAddress() + 0);
        Ok(b)
    }

    // https://wiki.osdev.org/Serial_Ports#Line_Status_Register
    unsafe fn isTransmitNotEmpty(&self) -> bool {
        let mut val = inB(self.port.getPortAddress() + 5);

        // Bit is set if transmition buffer is empty
        val = val & 0x20;

        if val == 0 {
            true
        } else {
            false
        }
    }

    unsafe fn init(&self) -> Result<bool, SerialFailure> {
        // BUGBUG: For yet another reason I do not understand if we don't have a random logging statment around the first few lines, we'll reset the CPU on real hardware (unsure of the fault)
        vgaWriteLine!("Serial init...");
        self.enableInterrupts(false);
        self.set115KBaud();
        self.setupFIFO();
        self.enableLoopback();

        // See if we can send a byte succesfully through loopack to validate
        // this thing is working
        let testByte = 0xDA;
        self.sendByte(testByte)?;
        let receivedByte = self.receiveByte()?;

        if testByte != receivedByte {
            vgaWriteLine!(
                "0x{:X} != 0x{:X}. Port {:?} is no good.",
                testByte,
                receivedByte,
                self.port
            );
            return Ok(false);
        }

        self.disableLoopback();
        Ok(true)
    }

    // https://wiki.osdev.org/Serial_Ports#Interrupt_enable_register
    unsafe fn enableInterrupts(&self, enable: bool) {
        if enable {
            todo!("Enable interrupts")
        } else {
            outB(self.port.getPortAddress() + 1, 0x00);
        }
    }

    unsafe fn set115KBaud(&self) {
        // Indicate we want to muck with the Divisor Latch Access Bit (DLAB) register
        outB(self.port.getPortAddress() + 3, 0x80);

        // Set divisor to 1
        outB(self.port.getPortAddress() + 0, 0x01); // low byte
        outB(self.port.getPortAddress() + 1, 0x00); // high byte

        // Clear mucking with DLAB bit, but also set no parity, one stop bit
        outB(self.port.getPortAddress() + 3, 0x03);
    }

    // https://wiki.osdev.org/Serial_Ports#First_In_First_Out_Control_Register
    unsafe fn setupFIFO(&self) {
        // Setting to 1 byte triggering for now in an attempt to have maximum reliablity
        outB(self.port.getPortAddress() + 2, 0x07);
    }

    // https://wiki.osdev.org/Serial_Ports#Modem_Control_Register
    unsafe fn enableLoopback(&self) {
        // Enable loop back
        // Sets Data Terminal Ready (DTR) & Request to Send (RTS)
        outB(self.port.getPortAddress() + 4, 0x0B);

        // Still enables loopback, but now enable the two control pins
        // and only RTS (not DTR)
        // BUGBUG: Rationalize this copy/paste from osdev, why can't this be done
        // in one shot, and-or does there need to be a delay between these two commands
        outB(self.port.getPortAddress() + 4, 0x1E);
    }

    unsafe fn dataAvailable(&self) -> bool {
        let mut val = inB(self.port.getPortAddress() + 5);
        val &= 0x1;

        val != 0
    }

    // https://wiki.osdev.org/Serial_Ports#Modem_Control_Register
    unsafe fn disableLoopback(&self) {
        // Clears loopback, enables both hardware pins and sets DTR, RTS
        outB(self.port.getPortAddress() + 4, 0x0F);
    }
}

impl COMPort {
    fn getPortAddress(&self) -> u16 {
        match self {
            COMPort::COM1 => 0x3F8,
        }
    }
}
