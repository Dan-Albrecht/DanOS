use super::virtualMemory::VirtualMemoryManager;

pub struct VirtualAddress {
    address: usize,
}

pub struct PhysicalAddress {
    pub address: usize,
}

impl VirtualAddress {
    pub fn new(address: usize) -> Self {
        VirtualAddress { address }
    }

    pub fn toPhysical(&self, vmm: &VirtualMemoryManager) -> Option<PhysicalAddress> {
        let result = vmm.getPhysical(self.address);

        if result.is_none() {
            return None;
        }

        return Some(PhysicalAddress {
            address: result.unwrap(),
        });
    }
}

impl PhysicalAddress {
    pub fn new(address: usize) -> Self {
        PhysicalAddress { address }
    }
}
