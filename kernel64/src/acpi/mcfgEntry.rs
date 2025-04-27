use crate::{
    acpi::{
        pciCommonHeader::{PciCommonHeader, PciHeaderType},
        pciGeneralDevice::PciGeneralDevice,
    }, loggerWriteLine
};

// Same no UEFI docs story as MCFG
// https://wiki.osdev.org/PCI_Express#Enhanced_Configuration_Mechanism
// Also useful https://wiki.qemu.org/images/f/f6/PCIvsPCIe.pdf.
#[repr(C, packed)]
pub struct McfgEntry {
    pub BaseAddress: u64,
    pub SegmentGroup: u16,
    pub StartBus: u8,
    pub EndBus: u8,
    pub Reserved: u32,
}

impl McfgEntry {
    pub fn printSomeInfo(&self) -> Option<*const PciGeneralDevice> {
        let base = self.BaseAddress;
        let seg = self.SegmentGroup;
        let start = self.StartBus;
        let end = self.EndBus;

        let mut ahciController: Option<*const PciGeneralDevice> = None;

        loggerWriteLine!(
            "    Base 0x{:X} for group {} busses {}..={}",
            base,
            seg,
            start,
            end
        );

        for device in 0..32 {
            match PciCommonHeader::tryGetEntry(&self, 0, device, 0) {
                Some(header) => unsafe {
                    let headerType = (*header).getType();
                    let result =
                        self.printDetails(header, &headerType, device, 0, (*header).ProgIF);

                    // First one wins
                    if ahciController == None {
                        if let Some(result) = result {
                            ahciController = Some(result);
                        }
                    }

                    if headerType == PciHeaderType::MultiFunctionGeneral {
                        for remainingFunction in 1..=7 {
                            match PciCommonHeader::tryGetEntry(&self, 0, device, remainingFunction)
                            {
                                Some(innerHeader) => {
                                    let headerType = (*innerHeader).getType();
                                    let result = self.printDetails(
                                        innerHeader,
                                        &headerType,
                                        device,
                                        remainingFunction,
                                        (*header).ProgIF,
                                    );
                                    
                                    // First one wins
                                    if ahciController == None {
                                        if let Some(result) = result {
                                            ahciController = Some(result);
                                        }
                                    }
                                }
                                None => {}
                            }
                        }
                    }
                },
                None => (),
            }
        }

        return ahciController;
    }

    unsafe fn printDetails(
        &self,
        header: *const PciCommonHeader,
        headerType: &PciHeaderType,
        device: u8,
        function: u8,
        progIF: u8,
    ) -> Option<*const PciGeneralDevice> { unsafe {
        let cc = (*header).ClassCode;
        let sc = (*header).Subclass;

        if *headerType == PciHeaderType::Dunno {
            loggerWriteLine!("    Dunno is 0x{:X}", (*header).HeaderType);
        }

        loggerWriteLine!(
            "    Device {} Function {} exists as a {:?} 0x{:X}:0x{:X}:0x{:X}",
            device,
            function,
            headerType,
            cc,
            sc,
            progIF
        );

        let maybeDevice = PciGeneralDevice::tryGet(&*header);
        if let Some(device) = maybeDevice {
            (*device).printBars();

            // Class - Mass Storage Controller
            // Subclass - Serial ATA Controller
            // Interface - AHCI
            if cc == 0x1 && sc == 0x6 && progIF == 0x1 {
                return Some(device);
            }
        }

        return None;
    }}
}
