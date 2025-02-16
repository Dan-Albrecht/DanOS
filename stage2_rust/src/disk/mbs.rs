use kernel_shared::vgaWriteLine;

// Master Boot Record
pub struct Mbr {
    pub partition1: PartitionEntry,
    pub partition2: PartitionEntry,
    pub partition3: PartitionEntry,
    pub partition4: PartitionEntry,
}

pub struct PartitionEntry {
    pub bootable: bool,
    pub start_head: u8,
    pub start_sector: u8,
    pub start_cylinder: u8,
    pub partition_type: u8,
    pub end_head: u8,
    pub end_sector: u8,
    pub end_cylinder: u8,
    pub start_lba: u32,
    pub size: u32,
}

impl Mbr {
    pub fn new(buffer: &[u8]) -> Result<Self, &'static str> {
        // BUGBUG: Would like to restrict the argument to just a reference to the last 66 bytes of the buffer

        if buffer.len() < 66 {
            return Err("Buffer must be at least 66-bytes to get the data we need");
        }

        let signature = &buffer[buffer.len() - 2..];
        if signature != [0x55, 0xAA] {
            return Err("Invalid MBR signature");
        }

        let mut partionInfoStart = buffer.len() - 66;

        let partition1 = Self::getPE(buffer, partionInfoStart);
        partionInfoStart += 16;
        let partition2 = Self::getPE(buffer, partionInfoStart);
        partionInfoStart += 16;
        let partition3 = Self::getPE(buffer, partionInfoStart);
        partionInfoStart += 16;
        let partition4 = Self::getPE(buffer, partionInfoStart);

        Ok(Self {
            partition1,
            partition2,
            partition3,
            partition4,
        })
    }

    pub fn dumpActive(&self) {
        if self.partition1.bootable {
            self.dumpPartition("1", &self.partition1);
        }
        if self.partition2.bootable {
            self.dumpPartition("2", &self.partition2);
        }
        if self.partition3.bootable {
            self.dumpPartition("3", &self.partition3);
        }
        if self.partition4.bootable {
            self.dumpPartition("4", &self.partition4);
        }
    }

    fn getPE(buffer: &[u8], offset: usize) -> PartitionEntry {
        PartitionEntry {
            bootable: buffer[offset + 0] == 0x80,
            start_head: buffer[offset + 1],
            start_sector: buffer[offset + 2] & 0b0011_1111,
            start_cylinder: (buffer[offset + 2] & 0b1100_0000) << 2 | buffer[offset + 3],
            partition_type: buffer[offset + 4],
            end_head: buffer[offset + 5],
            end_sector: buffer[offset + 6] & 0b0011_1111,
            end_cylinder: (buffer[offset + 6] & 0b1100_0000) << 2 | buffer[offset + 7],
            start_lba: u32::from_le_bytes([buffer[offset + 8], buffer[offset + 9], buffer[offset + 10], buffer[offset + 11]]),
            size: u32::from_le_bytes([buffer[offset + 12], buffer[offset + 13], buffer[offset + 14], buffer[offset + 15]]),
        }
    }
    
    fn dumpPartition(&self, arg: &str, pe: &PartitionEntry) {
        vgaWriteLine!("Partition {} - Bootable: {}", arg, pe.bootable);
        vgaWriteLine!("  Start Head: {}", pe.start_head);
        vgaWriteLine!("  Start Sector: {}", pe.start_sector);
        vgaWriteLine!("  Start Cylinder: {}", pe.start_cylinder);
        vgaWriteLine!("  Partition Type: {}", pe.partition_type);
        vgaWriteLine!("  End Head: {}", pe.end_head);
        vgaWriteLine!("  End Sector: {}", pe.end_sector);
        vgaWriteLine!("  End Cylinder: {}", pe.end_cylinder);
        vgaWriteLine!("  Start LBA: {}", pe.start_lba);
        vgaWriteLine!("  Size: {}", pe.size);
    }
}
