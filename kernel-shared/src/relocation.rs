use core::ptr::{slice_from_raw_parts, write_unaligned};
use elf::{ElfBytes, abi::R_X86_64_RELATIVE, endian::NativeEndian, file::Class};

use crate::{assemblyStuff::halt::haltLoop, haltLoopWithMessage, loggerWriteLine, vgaWriteLine};

pub unsafe fn relocateKernel64(elfStartAddress: usize, length: usize) -> usize {
    unsafe { relocateKernel64Ex(elfStartAddress, length, elfStartAddress) }
}

// Returns the location where the .text section is
pub unsafe fn relocateKernel64Ex(
    elfStartAddress: usize,
    length: usize,
    targetAddress: usize,
) -> usize {
    unsafe {
        let length: usize = length.try_into().unwrap();
        vgaWriteLine!("Parsing ELF @ 0x{:X} for 0x{:X}", elfStartAddress, length);
        let data = slice_from_raw_parts(elfStartAddress as *const u8, length);
        let elf = ElfBytes::<NativeEndian>::minimal_parse(&*data);
        if elf.is_err() {
            let e = elf.unwrap_err();
            haltLoopWithMessage!("Parse failed: {}", e);
        }
        let elf = elf.expect("ELF parse failed");
        if elf.ehdr.class != Class::ELF64 {
            haltLoopWithMessage!("Expected 64bit elf, got: {:?}", elf.ehdr.class);
        }

        let textSection = elf
            .section_header_by_name(".text")
            .expect("Couldn't parse section table")
            .expect("Couldn't find .text section");
        let textOffset = textSection.sh_offset;
        let kernelThinks = textSection.sh_addr;
        let actualTextLocation: u64 = textOffset + elfStartAddress as u64;
        let desiredTextLocation: u64 = textOffset + targetAddress as u64;

        let sourceRelocationAdjustment = actualTextLocation.wrapping_sub(kernelThinks);
        let targetRelocationAdjustment = desiredTextLocation.wrapping_sub(kernelThinks);

        loggerWriteLine!(
            "Kernel compiled to 0x{:X}, loaded from 0x{:X}, and we're relocating to 0x{:X} which is an adjustment of 0x{:X}",
            kernelThinks,
            actualTextLocation,
            desiredTextLocation,
            targetRelocationAdjustment
        );

        if actualTextLocation != kernelThinks {
            vgaWriteLine!(
                "Kernel thinks it's @ 0x{:X}, but is @ {:X}",
                kernelThinks,
                actualTextLocation
            );
        }

        let sections = elf.section_headers().expect("No ELF headers...");
        let mut relocationCount = 0;

        for section in sections.iter() {
            if let Ok(relocations) = elf.section_data_as_relas(&section) {
                for relocation in relocations {
                    if relocation.r_sym == 0 {
                        // BUGBUG: This library sucks, no idea what constant to use, just know this is 0. Might be STB_LOCAL though the size is wrong.
                        // Good relocation tutorial: https://fasterthanli.me/series/making-our-own-executable-packer/part-17
                        if relocation.r_type == R_X86_64_RELATIVE {
                            let offset = relocation.r_offset;
                            let addend = relocation.r_addend;
                            let result = targetRelocationAdjustment.wrapping_add(addend as u64);
                            let target = offset.wrapping_add(sourceRelocationAdjustment) as *mut u64;

                            write_unaligned(target, result);
                            relocationCount += 1;
                        } else {
                            haltLoopWithMessage!(
                                "Don't know how to do a {} relocation",
                                relocation.r_type
                            );
                        }
                    } else {
                        haltLoopWithMessage!("Don't know how to relocate a {}", relocation.r_sym);
                    }
                }
            }
        }

        vgaWriteLine!("Relocated {} entries", relocationCount,);

        return desiredTextLocation
            .try_into()
            .expect("Kernel64 .text offset");
    }
}
