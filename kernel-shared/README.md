# Kernel-Shared

This library is for the **minimal** code sharing between the 32-bit and 64-bit portions of the kernel. This code should all be `usize` based when size changes per architecture and only use fixed sizes when the size is the same across architectures (e.g output ports). `#![cfg(target_pointer_width = "XX")]` should be minimally used unless the same code is really going to appear in both places.  As this code will be used in the 32-bit 'kernel' it'll have no alocation abilties or access to all the good stuff in the 64-bit kernel.
