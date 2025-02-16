use kernel_shared::vgaWriteLine;

use super::{diskDriver::DiskDriver, mbs::Mbr};

pub struct FatDriver {
    disk: DiskDriver,
}

impl FatDriver {
    pub fn new(disk: DiskDriver) -> Self {
        FatDriver { disk }
    }

    pub fn findKernel32(&self) -> Result<usize, &'static str> {
        let mut buffer = [0 as u8; 512];

        // Read the first sector of the disk so we can get partition information
        self.disk.read(0, &mut buffer)?;

        let mbr = Mbr::new(&buffer)?;
        mbr.dumpActive();
        Ok(0)
    }
}
