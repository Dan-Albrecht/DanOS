use core::arch::asm;

use kernel_shared::{vgaWrite, vgaWriteLine};

use super::mapEntry::MemoryMapEntry;

pub struct MemoryMap {
    pub Entries: [MemoryMapEntry; 32],
    pub EntryCount: u8,
}

impl MemoryMap {
    pub unsafe fn create() -> Result<MemoryMap, &'static str> {
        let smapSignature: u32 = "SMAP".chars().fold(0, |accumulator, currentChar| {
            (accumulator << 8) | currentChar as u32
        });

        let function: u32 = 0xE820;
        let bufferSize: u32 = size_of::<MemoryMapEntry>().try_into().unwrap();

        let mut eax: u32;
        let mut continuation: u32 = 0;
        let mut ecx: u32;
        let edx: u32 = smapSignature;

        let mut result = Self::new();
        let mut diVal: *mut _;
        let mut firstTime = true;

        loop {
            eax = function;
            ecx = bufferSize;
            let currentIndex = result.EntryCount as usize;
            diVal = core::ptr::from_mut(&mut result.Entries[currentIndex]);

            unsafe {
                asm!(
                    "int 0x15",
                    in("di") diVal,
                    inout("eax") eax,
                    inout("ebx") continuation,
                    inout("ecx") ecx,
                    in("edx") edx,
                );
            }

            _ = ecx;

            if firstTime {
                firstTime = false;

                // Seems a bit cumbersome to check the carry flag for success, so we're just
                // going to rely on checking the signature only.
                if eax != smapSignature {
                    vgaWriteLine!("Unexpected signature 0x{:X}", eax);
                    return Err("int 0x15, 0xE820 not supported");
                }
            } else {
                result.EntryCount += 1;

                // And here again, no CF, will just look at continuation saying we're done.
                if continuation == 0 {
                    return Ok(result);
                }

                if result.EntryCount as usize >= result.Entries.len() {
                    return Err("Too many entries");
                }
            }
        }
    }

    fn new() -> Self {
        MemoryMap {
            Entries: [MemoryMapEntry {
                BaseAddress: 0,
                Length: 0,
                Type: 0,
                ExtendedAttributes: 0,
            }; 32],
            EntryCount: 0,
        }
    }

    pub fn dump(&self){
        // We do this backwards, because if there's a crash we'll hope the first few entries are on the screen
        // (especially if we don't have serial hooked up)
        for index in (0..self.EntryCount as usize).rev() {
            vgaWrite!("{}: ", index);
            self.Entries[index].dump();
        }
    }
}
