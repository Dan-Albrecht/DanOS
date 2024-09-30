pub const VM_KERNEL64_CODE: usize = 0x20_0000;
pub const VM_KERNEL64_DATA: usize = 0x40_0000;

pub const VM_KERNEL64_CODE_LENGTH: usize = 0x10_0000;

// BUGBUG: Validate data is > stack
pub const VM_KERNEL64_STACK_LENGTH: usize = 0x10_0000;
pub const VM_KERNEL64_DATA_LENGTH: usize = 0x20_0000;
