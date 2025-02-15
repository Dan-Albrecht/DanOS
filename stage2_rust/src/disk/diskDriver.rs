use kernel_shared::vgaWriteLine;



// Disk Address Packet Structure
#[repr(C, packed)]
struct DAPS {
    size: u8,
    zero: u8,
    sectorsToRead: u16,
    readToOffset: u16,
    readToSegment: u16,
    lbaStart: u64,
}

pub struct DiskDriver {
    drive: u32, // BIOS drive number
}

impl DiskDriver {
    pub fn new(drive: u32) -> Self {
        DiskDriver { drive }
    }

    pub fn doStuff(&self) {
        vgaWriteLine!("Using disk 0x{:X}", self.drive);

        let _d = DAPS {
            size: 0x10,
            zero: 0x0,
            sectorsToRead: 0x10,
            readToOffset: 0x10,
            readToSegment: 0x10,
            lbaStart: 0x10,
        };
    }
}
