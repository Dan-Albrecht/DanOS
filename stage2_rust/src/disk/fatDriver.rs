use kernel_shared::{vgaWrite, vgaWriteLine};

use super::{diskDriver::DiskDriver, mbs::Mbr};

pub struct FatDriver {
    disk: DiskDriver,
}

// BIOS Parameter Block
#[repr(C, packed)]
struct Bpb {
    jmp: [u8; 3], // Machine code to jump past this incase someone tried to execute it
    oem: [u8; 8], // OEM name
    bytes_per_sector: u16,
    sectors_per_cluster: u8,
    reserved_sectors: u16,
    fat_count: u8,
    root_entries: u16,
    total_sectors: u16, // If this is 0, then use total_sectors_large
    media_descriptor: u8,
    sectors_per_fat: u16,
    sectors_per_track: u16,
    head_count: u16,
    hidden_sectors: u32,
    total_sectors_large: u32,
    _unused: [u8; 18],
    file_system_type: [u8; 8],
}

struct Fat16 {
    bpb: Bpb,
    base_lba: u32, // LBA of the first sector of the partition
}

const ATTR_READ_ONLY: u8 = 0x01;
const ATTR_HIDDEN: u8 = 0x02;
const ATTR_SYSTEM: u8 = 0x04;
const ATTR_VOLUME_ID: u8 = 0x08;
const ATTR_DIRECTORY: u8 = 0x10;
const ATTR_ARCHIVE: u8 = 0x20;

#[repr(C, packed)]
struct DirectoryEntry {
    pub name: [u8; 8],
    pub ext: [u8; 3],
    pub attributes: u8,
    pub _unused: [u8; 20],
}

impl Fat16 {
    pub fn new(bpb: Bpb, base_lba: u32) -> Self {
        Self { bpb, base_lba }
    }

    pub fn doSomething(&self, disk: &DiskDriver) -> Result<(), &'static str> {
        const FAT_ENTRIES: usize = 512;
        let mut buffer = [0 as u8; FAT_ENTRIES * 2];

        // BUGBUG: This casting is annoying; figure out the correct way that doesn't involve 'as'
        let mut start = self.base_lba as u64;
        let dumb: u64 = self.bpb.reserved_sectors.into();
        start += dumb;

        disk.read(start, &mut buffer)?;

        let _fatTable =
            unsafe { core::slice::from_raw_parts(&buffer as *const _ as *const u16, FAT_ENTRIES) };

        self.dumpRootDirectory(disk)?;

        Ok(())
    }

    fn dumpRootDirectory(&self, disk: &DiskDriver) -> Result<(), &'static str> {
        let rootDirStartSector =
            self.bpb.reserved_sectors + (self.bpb.fat_count as u16 * self.bpb.sectors_per_fat);
        let rootDirStartLba = self.base_lba + rootDirStartSector as u32;
        let dirSize = core::mem::size_of::<DirectoryEntry>() as u32;
        let rootDirSize = ((self.bpb.root_entries as u32 * dirSize)
            + (self.bpb.bytes_per_sector as u32 - 1))
            / self.bpb.bytes_per_sector as u32;

        vgaWriteLine!(
            "Root directory starts at LBA 0x{:X} and is {} sectors and contains:",
            rootDirStartLba,
            rootDirSize
        );

        let mut buffer = [0 as u8; 512];
        disk.read(rootDirStartLba.into(), &mut buffer)?;
        let dirs : [DirectoryEntry; 16] = unsafe { core::mem::transmute(buffer) };
        for dir in dirs.iter() {
            if dir.attributes & ATTR_VOLUME_ID != 0 {
                vgaWriteLine!(" Volume ID / Root Directory");
                continue;
            }
            if dir.name[0] == 0 {
                // End of data
                break;
            }
            
            if dir.name[0] == 0xE5 {
                // This entry is free, but there might be more
                continue;
            }

            if dir.attributes & ATTR_DIRECTORY != 0 {
                vgaWrite!(" Directory: ");
            } else {
                vgaWrite!(" File: ");
            }

            let name = core::str::from_utf8(&dir.name).unwrap_or("Invalid UTF-8");
            let ext = core::str::from_utf8(&dir.ext).unwrap_or("Invalid UTF-8");
            vgaWriteLine!(" {}.{}", name, ext);
        }

        Ok(())
    }
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
        let psResult = mbr.getActivePartition()?;

        if psResult.is_none() {
            return Err("No active partition found");
        }

        let peResult = psResult.unwrap();
        let pe = peResult.0;
        let partitionNumber = peResult.1;
        pe.dumpPartition(partitionNumber);

        let bpb = self.readBpb(pe)?;
        let fat16 = Fat16::new(bpb, pe.start_lba);
        fat16.doSomething(&self.disk)?;

        Ok(0)
    }

    fn readBpb(&self, pe: &super::mbs::PartitionEntry) -> Result<Bpb, &'static str> {
        const BPB_SIZE: usize = core::mem::size_of::<Bpb>();
        let mut buffer = [0 as u8; 512];

        self.disk.read(pe.start_lba.into(), &mut buffer)?;
        let bpb_bytes = &buffer as *const _ as *const [u8; BPB_SIZE];
        let bpb: Bpb = unsafe { core::mem::transmute(*bpb_bytes) };
        bpb.dump();

        if bpb.root_entries == 0 {
            return Err("Root entries is 0; so this is probably FAT32");
        }

        if bpb.sectors_per_fat == 0 {
            return Err("Sectors per FAT is 0; so this is probably FAT32");
        }

        if bpb.total_sectors == 0 {
            return Err("Total sectors is 0; so this is probably FAT32");
        }

        let rootDirSectores =
            ((bpb.root_entries * 32) + (bpb.bytes_per_sector - 1)) / bpb.bytes_per_sector;
        let dataSector = bpb.total_sectors
            - (bpb.reserved_sectors as u16
                + (bpb.fat_count as u16 * bpb.sectors_per_fat)
                + rootDirSectores);
        let countOfClusters = dataSector / bpb.sectors_per_cluster as u16;

        if countOfClusters < 4085 {
            return Err("Count of clusters is less than 4085; so this is FAT12");
        }

        if countOfClusters >= 65525 {
            return Err("Count of clusters is greater than 65525; so this is FAT32");
        }

        vgaWriteLine!("Disk appears to be FAT16 with {} clusters", countOfClusters);

        Ok(bpb)
    }
}

impl Bpb {
    pub fn dump(&self) {
        let oem_str = core::str::from_utf8(&self.oem).unwrap_or("Invalid UTF-8");
        vgaWriteLine!("OEM: {}", oem_str);
        vgaWriteLine!("Bytes per sector: {}", { self.bytes_per_sector });
        vgaWriteLine!("Sectors per cluster: {}", { self.sectors_per_cluster });
        vgaWriteLine!("Reserved sectors: {}", { self.reserved_sectors });
        vgaWriteLine!("FAT count: {}", { self.fat_count });
        vgaWriteLine!("Root entries: {}", { self.root_entries });
        vgaWriteLine!("Total sectors: {}", { self.total_sectors });
        vgaWriteLine!("Media descriptor: {}", { self.media_descriptor });
        vgaWriteLine!("Sectors per FAT: {}", { self.sectors_per_fat });
        vgaWriteLine!("Sectors per track: {}", { self.sectors_per_track });
        vgaWriteLine!("Head count: {}", { self.head_count });
        vgaWriteLine!("Hidden sectors: {}", { self.hidden_sectors });
        vgaWriteLine!("Total sectors large: {}", { self.total_sectors_large });
        let file_system_type =
            core::str::from_utf8(&self.file_system_type).unwrap_or("Invalid UTF-8");
        vgaWriteLine!("File system type: {}", file_system_type);
    }
}
