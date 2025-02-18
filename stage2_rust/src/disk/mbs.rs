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

    pub fn dumpPartitions(&self) {
        self.partition1.dumpPartition(1);
        self.partition2.dumpPartition(2);
        self.partition3.dumpPartition(3);
        self.partition4.dumpPartition(4);
    }

    pub fn getActivePartition(&self) -> Result<Option<(&PartitionEntry, u8)>, &'static str> {
        let mut result: Option<(&PartitionEntry, u8)> = None;

        if self.partition1.bootable {
            result = Some((&self.partition1, 1));
        }

        if self.partition2.bootable {
            if result.is_some() {
                return Err("Multiple bootable partitions found");
            }
            result = Some((&self.partition2, 2));
        }

        if self.partition3.bootable {
            if result.is_some() {
                return Err("Multiple bootable partitions found");
            }
            result = Some((&self.partition3, 3));
        }

        if self.partition4.bootable {
            if result.is_some() {
                return Err("Multiple bootable partitions found");
            }
            result = Some((&self.partition4, 4));
        }

        Ok(result)
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
            start_lba: u32::from_le_bytes([
                buffer[offset + 8],
                buffer[offset + 9],
                buffer[offset + 10],
                buffer[offset + 11],
            ]),
            size: u32::from_le_bytes([
                buffer[offset + 12],
                buffer[offset + 13],
                buffer[offset + 14],
                buffer[offset + 15],
            ]),
        }
    }
}

impl PartitionEntry {
    pub fn dumpPartition(&self, partitionNumber: u8) {
        vgaWriteLine!(
            "Partition {} - Bootable: {}",
            partitionNumber,
            self.bootable
        );
        vgaWriteLine!("  Start Head: {}", self.start_head);
        vgaWriteLine!("  Start Sector: {}", self.start_sector);
        vgaWriteLine!("  Start Cylinder: {}", self.start_cylinder);
        vgaWriteLine!("  Partition Type: {}", self.partition_type);
        vgaWriteLine!("  End Head: {}", self.end_head);
        vgaWriteLine!("  End Sector: {}", self.end_sector);
        vgaWriteLine!("  End Cylinder: {}", self.end_cylinder);
        vgaWriteLine!("  Start LBA: {}", self.start_lba);
        vgaWriteLine!("  Size: {}", self.size);
    }
}
