#![no_std]
#![allow(non_snake_case)]
#![feature(if_let_guard)]
#![feature(core_intrinsics)]

pub mod alignment;
pub mod assemblyStuff;
pub mod gdtStuff;
pub mod magicConstants;
pub mod memoryHelpers;
pub mod memoryMap;
pub mod pageTable;
pub mod physicalMemory;
pub mod textMode;
