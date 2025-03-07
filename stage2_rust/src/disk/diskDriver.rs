use core::arch::asm;

use kernel_shared::vgaWriteLine;

// Disk Address Packet Structure
#[repr(C, packed)]
struct DAPS {
    structSize: u8,
    zero: u8,
    sectorsToRead: u16,
    readToOffset: u16,
    readToSegment: u16,
    lbaStart: u64,
}

pub struct DiskDriver {
    drive: u8, // BIOS drive number
}

impl DiskDriver {
    pub fn new(drive: u8) -> Self {
        DiskDriver { drive }
    }

    // Read sectors from disk. The disk is read starting from the LBA address
    // into the buffer for the full length of the buffer. The buffer's length
    // must be a multiple of 512 bytes.
    pub fn read(&self, lba: u64, buffer: &mut [u8]) -> Result<(), &'static str> {
        if (buffer.as_ptr() as usize) > u16::MAX as usize {
            return Err("Buffer address overflow");
        }

        if buffer.len() % 512 != 0 {
            return Err("Buffer length must be a multiple of 512 bytes");
        }

        let bufferAddress = buffer.as_mut_ptr() as u16;

        let mut daps = DAPS {
            structSize: size_of::<DAPS>().try_into().unwrap(),
            zero: 0,
            sectorsToRead: (buffer.len() / 512).try_into().unwrap(),
            readToOffset: bufferAddress,
            readToSegment: 0,
            lbaStart: lba,
        };

        let dapsAddress: usize = &mut daps as *mut _ as usize;
        if dapsAddress > u16::MAX as usize {
            vgaWriteLine!("DAP Pointer address overflow: 0x{:X}", dapsAddress);
            return Err("DAP address overflow");
        }
        let dapsAddress = dapsAddress as u16;
        let mut ah: u8 = 0x42; // Extended Read function

        // https://en.wikipedia.org/wiki/INT_13H#INT_13h_AH=42h:_Extended_Read_Sectors_From_Drive
        unsafe {
            asm!(
                "push si",
                "mov si, bx",
                "int 0x13",
                "pop si",
                inout("ah") ah,
                in("dl") self.drive,
                in("bx") dapsAddress,
            );
        }

        if ah != 0 {
            vgaWriteLine!("Disk read error: 0x{:X}", ah);
            return Err("Disk read error");
        }

        Ok(())
    }
}
