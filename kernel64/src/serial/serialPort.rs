
use core::fmt::Write;
use kernel_shared::{assemblyStuff::ports::{inB, outB}, vgaWriteLine};

use crate::loggerWriteLine;

pub struct SerialPort {
    port: COMPort,
}

#[derive(Debug)]
pub enum COMPort {
    COM1,
}

// https://wiki.osdev.org/Serial_Ports
impl SerialPort {
    pub fn tryGet(port: COMPort) -> Option<SerialPort> {
        let result = SerialPort { port: port };

        unsafe {
            if result.init() {
                Some(result)
            } else {
                None
            }
        }
    }

    pub fn SendLine(&self, msg: &[u8]) {
        self.Send(msg);

        unsafe {
            // CR, LF
            self.sendByte(0xD);
            self.sendByte(0xA);
        }
    }

    pub fn Send(&self, msg: &[u8]) {
        for index in 0..msg.len() {
            unsafe {
                self.sendByte(msg[index]);
            }
        }
    }

    unsafe fn sendByte(&self, b: u8) {
        while self.isTransmitNotEmpty() {}
        outB(self.port.getPortAddress() + 0, b);
    }

    unsafe fn receiveByte(&self) -> u8 {
        while !self.dataAvailable() {
            
        }

        let b = inB(self.port.getPortAddress() + 0);
        b
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

    unsafe fn init(&self) -> bool {
        self.enableInterrupts(false);
        self.set115KBaud();
        self.setupFIFO();
        self.enableLoopback();
        
        // See if we can send a byte succesfully through loopack to validate
        // this thing is working
        let testByte = 0xDA;
        self.sendByte(testByte);
        let receivedByte = self.receiveByte();

        if testByte != receivedByte {
            loggerWriteLine!("0x{:X} != 0x{:X}. Port {:?} is no good.", testByte, receivedByte, self.port);
            return false;
        }

        self.disableLoopback();
        true
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
