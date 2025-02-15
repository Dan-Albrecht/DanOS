use kernel_shared::vgaWriteLine;

use super::diskDriver::DiskDriver;

pub struct FatDriver {
    _disk: DiskDriver,
}

impl FatDriver {
    pub fn new(disk: DiskDriver) -> Self {
        FatDriver { _disk: disk }
    }

    pub fn doStuff(&self) {
        vgaWriteLine!("FAT stuff");
    }
}
