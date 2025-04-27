// Virtual address of the final location of the kernel
// This include the ELF header, the text section immediately follows
pub const VM_KERNEL64_ELF: usize = 0x2000_0000;

// Virtual address of the final location of the kernel's data space
pub const VM_KERNEL64_DATA: usize = 0x4000_0000;

// BUGBUG: Validate data is > stack
// The current intent is data is stack + heap. The stack will take up the first part and grown down, and the heap will take up the rest and grow up.
pub const VM_KERNEL64_STACK_LENGTH: usize = 0x10_0000;
pub const VM_KERNEL64_DATA_LENGTH: usize = 0x20_0000;

pub const DUMB_HEAP_SIZE: usize = 0x5_0000;