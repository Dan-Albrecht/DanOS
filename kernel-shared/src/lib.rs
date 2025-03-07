#![no_std]
#![allow(non_snake_case)]
#![feature(if_let_guard)]
#![feature(core_intrinsics)]

pub mod alignment;
pub mod assemblyStuff;
pub mod gdtStuff;
pub mod magicConstants;
pub mod memory;
pub mod memoryHelpers;
pub mod memoryTypes;
pub mod pageTable;
pub mod physicalMemory;
pub mod relocation;
pub mod textMode;
