use super::{diskDriver::DiskDriver, mbs::Mbr};
use core::{slice, str};
use enumflags2::{BitFlag, bitflags};
use kernel_shared::{haltLoopWithMessage, vgaWrite, vgaWriteLine};

// Sector - Unit of access for the media. We'll be using 512 bytes.
// Cluster - Multiple of Sector. Instrinct allocation unit for the file system.

pub struct FatDriver {
    disk: DiskDriver,
    f16: Option<Fat16>,
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
    sectors_per_fat: u16, // BPB_FATSz16
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

#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DirectoryEntryAttribute {
    ReadOnly = 0x01,
    Hidden = 0x02,
    System = 0x04,
    VolumeId = 0x08,
    Directory = 0x10,
    Archive = 0x20,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
struct DirectoryEntry {
    pub name: [u8; 8],
    pub ext: [u8; 3],
    pub attributes: u8,
    reserved: u8,
    pub creation_time_tenths: u8,
    pub creation_time: u16,
    pub creation_date: u16,
    pub last_access_date: u16,
    pub first_cluster_high: u16,
    pub last_write_time: u16,
    pub last_write_date: u16,
    pub first_cluster_low: u16,
    pub file_size: u32,
}

#[derive(Copy, Clone)]
pub struct FileInfo {
    pub name: [u8; 8],
    pub ext: [u8; 3],
    pub file_size: u32,
    pub first_cluster_low: u16,
}

impl Fat16 {
    pub fn new(bpb: Bpb, base_lba: u32) -> Self {
        Self { bpb, base_lba }
    }

    pub fn printFile(
        &self,
        disk: &DiskDriver,
        name: &[u8],
        ext: &[u8],
    ) -> Result<(), &'static str> {
        const FAT_ENTRIES: usize = 512;
        let mut buffer = [0 as u8; FAT_ENTRIES * 2];

        // BUGBUG: This casting is annoying; figure out the correct way that doesn't involve 'as'
        let mut start = self.base_lba as u64;
        let dumb: u64 = self.bpb.reserved_sectors.into();
        start += dumb;

        disk.read(start, &mut buffer)?;

        let fatTable: &[u16] =
            unsafe { core::slice::from_raw_parts(&buffer as *const _ as *const u16, FAT_ENTRIES) };

        let de = self.findFile(disk, false, Some((name, ext)))?;
        if de.is_none() {
            return Err("File not found");
        }

        let de = de.unwrap();

        if de.first_cluster_high != 0 {
            return Err("High cluster is not 0; so this is probably FAT32");
        }

        vgaWriteLine!("FAT0 is 0x{:X}", fatTable[0]);
        vgaWriteLine!("FAT1 is 0x{:X}", fatTable[1]);

        let root_dir_sectors = ((self.bpb.root_entries * 32) + (self.bpb.bytes_per_sector - 1))
            / self.bpb.bytes_per_sector;

        let first_data_sector = self.bpb.reserved_sectors
            + (self.bpb.fat_count as u16 * self.bpb.sectors_per_fat)
            + root_dir_sectors;

        let first_data_lba = self.base_lba + first_data_sector as u32;

        let max_cluster = (self.bpb.total_sectors / self.bpb.sectors_per_cluster as u16) as usize;
        let mut cluster = de.first_cluster_low as usize;
        let mut bytes_to_read = de.file_size as usize;

        loop {
            if bytes_to_read == 0 {
                return Err("Bytes to read is 0");
            }

            let fatEntry = fatTable[cluster];
            //vgaWriteLine!("<<Cluster {} has FAT entry 0x{:X}>>", cluster, fatEntry);

            if fatEntry == 0 {
                return Err("Unexpected free cluster");
            }

            if fatEntry == 1 {
                return Err("Unexpected reserved cluster value");
            }

            if fatEntry >= (max_cluster as u16) + 1 && fatEntry <= 0xFFF6 {
                return Err("Reserved cluster value");
            }

            if fatEntry == 0xFFF7 {
                return Err("Bad cluster");
            }

            // We're past all the bad cases, so we can read the cluster
            let cluster_lba =
                first_data_lba + (cluster as u32 - 2) * self.bpb.sectors_per_cluster as u32;

            vgaWrite!("<<Reading cluster {} at LBA 0x{:X}", cluster, cluster_lba);

            let mut buffer = [0 as u8; 1024]; // BUGBUG: Buffer should be size of sector and it not account for it with multiple reads for look at the next FAT entry
            disk.read(cluster_lba.into(), &mut buffer)?;
            let read_end;

            if bytes_to_read <= buffer.len() {
                read_end = bytes_to_read;
            } else {
                read_end = buffer.len();
            }

            bytes_to_read -= read_end; // BUGBUG: Underflow?

            vgaWriteLine!(" need 0x{:X} and 0x{:X} left>>", read_end, bytes_to_read);

            let data = str::from_utf8(&buffer[0..read_end]).unwrap_or("Invalid string data");
            let l = data.len();
            let c = data.chars().count();
            vgaWrite!("{}<<{},{}>>", data, l, c);

            if fatEntry >= 0xFFF8 && fatEntry <= 0xFFFE {
                // Reserved, but aparently can be treated as allocated and final cluster
                break;
            }

            if fatEntry == 0xFFFF {
                // Allocated and final cluster
                break;
            }

            cluster = fatEntry as usize;
        }

        vgaWriteLine!("<<End of file>>");

        if bytes_to_read != 0 {
            vgaWriteLine!("Bytes to read is 0x{:X} at end of file", bytes_to_read);
            return Err("Bytes to read is not 0 at end of file");
        }

        Ok(())
    }

    fn findFile(
        &self,
        disk: &DiskDriver,
        printInfo: bool,
        filename: Option<(&[u8], &[u8])>,
    ) -> Result<Option<DirectoryEntry>, &'static str> {
        let rootDirStartSector =
            self.bpb.reserved_sectors + (self.bpb.fat_count as u16 * self.bpb.sectors_per_fat);
        let rootDirStartLba = self.base_lba + rootDirStartSector as u32;
        let dirSize = core::mem::size_of::<DirectoryEntry>() as u32;
        let rootDirSize = ((self.bpb.root_entries as u32 * dirSize)
            + (self.bpb.bytes_per_sector as u32 - 1))
            / self.bpb.bytes_per_sector as u32;

        if printInfo {
            vgaWriteLine!(
                "Root directory starts at LBA 0x{:X} and is {} sectors and contains:",
                rootDirStartLba,
                rootDirSize
            );
        }

        const BUFFER_LENGTH: usize = 512;
        const ENTRIES_PER_SECTOR: usize = BUFFER_LENGTH / core::mem::size_of::<DirectoryEntry>();

        // Confirm the sizes are exact multiples
        const { assert!(BUFFER_LENGTH % core::mem::size_of::<DirectoryEntry>() == 0) };

        let mut buffer = [0 as u8; BUFFER_LENGTH];
        let mut lba: u64 = rootDirStartLba.into();
        let mut rootDirRead = 0;
        let mut done = false;
        let mut entriesRead = 0;

        while rootDirRead < rootDirSize && !done {
            disk.read(lba, &mut buffer)?;

            let entries: [DirectoryEntry; ENTRIES_PER_SECTOR] =
                unsafe { core::mem::transmute(buffer) };

            for entry in entries.iter() {
                entriesRead += 1;
                let attributes = DirectoryEntryAttribute::from_bits_truncate(entry.attributes);

                if attributes == DirectoryEntryAttribute::VolumeId {
                    if printInfo {
                        vgaWriteLine!(" Volume ID / Root Directory");
                    }
                    continue;
                }

                if attributes.contains(DirectoryEntryAttribute::VolumeId) {
                    if printInfo {
                        vgaWriteLine!(" Maybe a long file name entry");
                    }
                    continue;
                }

                if entry.name[0] == 0 {
                    // End of data
                    done = true;
                    if printInfo {
                        vgaWriteLine!("Finished reading root directory at entry {}", entriesRead);
                    }
                    break;
                }

                if entry.name[0] == 0xE5 {
                    // This entry is free, but there might be more
                    continue;
                }

                if printInfo {
                    if attributes.contains(DirectoryEntryAttribute::Directory) {
                        vgaWrite!(" Directory: ");
                    } else {
                        vgaWrite!(" File: ");
                    }
                }

                if printInfo || filename.is_some() {
                    let nameEnd = entry
                        .name
                        .iter()
                        .rposition(|&char| char != 0x20)
                        .unwrap_or(entry.name.len()-1);

                    let extEnd = entry
                        .ext
                        .iter()
                        .rposition(|&char| char != 0x20)
                        .unwrap_or(entry.ext.len()-1);

                    let name =
                        core::str::from_utf8(&entry.name[0..=nameEnd]).unwrap_or("Invalid Name");
                    let ext = core::str::from_utf8(&entry.ext[0..=extEnd]).unwrap_or("Invalid Ext");
                    let size = entry.file_size;

                    if printInfo {
                        vgaWriteLine!(" {}.{} - {} bytes", name, ext, size);
                    }

                    if let Some((file_name, file_ext)) = filename {
                        if name.as_bytes() == file_name && ext.as_bytes() == file_ext {
                            return Ok(Some(*entry));
                        }
                    }
                }
            }

            rootDirRead += BUFFER_LENGTH as u32;
            lba += 1;
        }

        Ok(None)
    }

    fn loadFile(
        &self,
        disk: &DiskDriver,
        kernel_info: &FileInfo,
        memory_target: &mut [u8],
    ) -> Result<(), &'static str> {

        // BUGBUG: Need to handle arbitrary number of entries
        const FAT_ENTRIES: usize = 8192;
        let mut fat_buffer = [0 as u8; FAT_ENTRIES * 2];

        // BUGBUG: This casting is annoying; figure out the correct way that doesn't involve 'as'
        let mut start = self.base_lba as u64;
        let dumb: u64 = self.bpb.reserved_sectors.into();
        start += dumb;

        disk.read(start, &mut fat_buffer)?;

        let fatTable: &[u16] =
            unsafe { slice::from_raw_parts(&fat_buffer as *const _ as *const u16, FAT_ENTRIES) };

            let root_dir_sectors = ((self.bpb.root_entries * 32) + (self.bpb.bytes_per_sector - 1))
            / self.bpb.bytes_per_sector;

        let first_data_sector = self.bpb.reserved_sectors
            + (self.bpb.fat_count as u16 * self.bpb.sectors_per_fat)
            + root_dir_sectors;

        let first_data_lba = self.base_lba + first_data_sector as u32;

        let max_cluster = (self.bpb.total_sectors / self.bpb.sectors_per_cluster as u16) as usize;
        let mut cluster_to_read = kernel_info.first_cluster_low as usize;
        let mut bytes_to_read = kernel_info.file_size as usize;
        let mut buffer_index_start = 0;
        
        let mut disk_buffer = [0 as u8; 1024];

        loop {
            if bytes_to_read == 0 {
                return Err("Bytes to read is 0");
            }

            let fatEntry = fatTable[cluster_to_read];
            //vgaWriteLine!("<<Cluster {} has FAT entry 0x{:X}>>", cluster_to_read, fatEntry);

            if fatEntry == 0 {
                return Err("Unexpected free cluster");
            }

            if fatEntry == 1 {
                return Err("Unexpected reserved cluster value");
            }

            if fatEntry >= (max_cluster as u16) + 1 && fatEntry <= 0xFFF6 {
                return Err("Reserved cluster value");
            }

            if fatEntry == 0xFFF7 {
                return Err("Bad cluster");
            }

            // We're past all the bad cases, so we can read the cluster
            let cluster_lba =
                first_data_lba + (cluster_to_read as u32 - 2) * self.bpb.sectors_per_cluster as u32;

            // BUGBUG: Buffer should be size of sector and it not account for it with multiple reads for look at the next FAT entry
            // BUGBUG: Also an issue where the memory buffer is only mod 512, not mod 1024
            let mut buffer_index_end = buffer_index_start;

            if bytes_to_read <= 1024 {
                // BUGBUG: Buffer reads need to be multiples of a sector
                buffer_index_end += 1024;
                bytes_to_read = 0;
            } else {
                buffer_index_end += 1024;
                bytes_to_read -= 1024;
            }

            //vgaWriteLine!("<<Reading cluster {} at LBA 0x{:X} to [{}..{}] with 0x{:X} left", cluster_to_read, cluster_lba, buffer_index_start, buffer_index_end, bytes_to_read);

            // Had trouble getting this to directly write to memory, so indirecting for now
            disk.read(cluster_lba.into(), &mut disk_buffer)?;
            memory_target[buffer_index_start..buffer_index_end].copy_from_slice(&disk_buffer);

            if fatEntry >= 0xFFF8 && fatEntry <= 0xFFFE {
                // Reserved, but aparently can be treated as allocated and final cluster
                break;
            }

            if fatEntry == 0xFFFF {
                // Allocated and final cluster
                break;
            }

            buffer_index_start += 1024;
            cluster_to_read = fatEntry as usize;
        }

        vgaWriteLine!("<<End of file>>");

        if bytes_to_read != 0 {
            vgaWriteLine!("Bytes to read is 0x{:X} at end of file", bytes_to_read);
            return Err("Bytes to read is not 0 at end of file");
        }

        Ok(())
    }
}

impl FatDriver {
    pub fn new(disk: DiskDriver) -> Result<Self, &'static str> {
        let mut buffer = [0 as u8; 512];

        // Read the first sector of the disk so we can get partition information
        disk.read(0, &mut buffer)?;

        let mbr = Mbr::new(&buffer)?;
        let psResult = mbr.getActivePartition()?;

        if psResult.is_none() {
            return Err("No active partition found");
        }

        let peResult = psResult.unwrap();
        let pe = peResult.0;
        let partitionNumber = peResult.1;
        pe.dumpPartition(partitionNumber);

        let mut result = FatDriver { disk, f16: None };

        let bpb = result.readBpb(pe)?;
        let fat16 = Fat16::new(bpb, pe.start_lba);
        fat16.findFile(&result.disk, true, None)?;
        result.f16 = Some(fat16);

        Ok(result)
    }

    pub fn printHiText(&self) -> Result<(), &'static str> {
        if self.f16.is_none() {
            return Err("No FAT16 driver found");
        }

        self.f16
            .as_ref()
            .unwrap()
            .printFile(&self.disk, b"HI", b"TXT")?;

        Ok(())
    }

    pub fn getFileInfo(&self, filename: (&[u8], &[u8])) -> Result<FileInfo, &'static str> {
        if self.f16.is_none() {
            return Err("No FAT16 driver found");
        }

        let de =
            self.f16
                .as_ref()
                .unwrap()
                .findFile(&self.disk, false, Some(filename))?;
        if de.is_none() {
            return Err("File not found");
        }

        let result = FileInfo {
            name: de.unwrap().name,
            ext: de.unwrap().ext,
            file_size: de.unwrap().file_size,
            first_cluster_low: de.unwrap().first_cluster_low,
        };

        Ok(result)
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

    // The address must be capable of holding the entire file rounded up to the nearest cluster
    pub unsafe fn loadFile(
        &self,
        address: usize,
        file_info: &FileInfo,
    ) -> Result<(), &'static str> {
        if self.f16.is_none() {
            return Err("No FAT16 driver found");
        }

        let needed_size = ((file_info.file_size as usize + 1023) / 1024) * 1024;

        unsafe {
            let buffer = slice::from_raw_parts_mut(address as *mut u8, needed_size);

            self.f16
                .as_ref()
                .unwrap()
                .loadFile(&self.disk, file_info, buffer)?;
        }

        Ok(())
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
        vgaWriteLine!("Total clusters: {}", {
            self.total_sectors / self.sectors_per_cluster as u16
        });
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
