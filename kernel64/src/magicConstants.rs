// Virtual address of the final location of the kernel
// This starts at the kernel entry point and does not include the ELF header like a lot of other stuff does
pub const VM_KERNEL64_CODE: usize = 0x2000_0000;

// Virtual address of the final location of the kernel's data space
pub const VM_KERNEL64_DATA: usize = 0x4000_0000;

// BUGBUG: Validate data is > stack
pub const VM_KERNEL64_STACK_LENGTH: usize = 0x10_0000;
pub const VM_KERNEL64_DATA_LENGTH: usize = 0x20_0000;

pub const DUMB_HEAP_SIZE: usize = 0x5_0000;