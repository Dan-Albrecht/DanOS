# DanOS
Another soon to be abanondone OS project. Written in Rust just to screw around.

## Useful Links

https://astralvx.com/debugging-16-bit-in-qemu-with-gdb-on-windows/
https://www.cs.cmu.edu/~pattis/15-1XX/common/handouts/ascii.html
https://blog.mattjustice.com/2018/08/24/gdb-for-windbg-users/
https://wiki.osdev.org/Text_UI
https://www.cs.bham.ac.uk/~exr/lectures/opsys/10_11/lectures/os-dev.pdf
https://os.phil-opp.com/freestanding-rust-binary/
https://docs.rust-embedded.org/embedonomicon/custom-target.html
https://stackoverflow.com/a/67902310
https://old.reddit.com/r/rust/comments/15yph7l/producing_a_completely_flat_binary_in_rust/
https://os.phil-opp.com/minimal-rust-kernel/

First edition has more low level details and jsut doesn't use a magic crate to dump you in 64bit mode
https://os.phil-opp.com/edition-1/

Custom linker setup might help with our unexpected layout (and giant size)
https://www.rustyelectrons.com/posts/1-bare-metal-rust-bootstrapped-by-c/

https://github.com/rust-osdev
https://github.com/rust-osdev/bootloader

## Creating empty.img

```
Create empty 6MB file
=====================
dd if=/dev/zero of=empty.img bs=6M count=1

Partition it
============
parted empty.img
mklabel msdos
mkpart primary fat16 1MiB 100%
<< above is where we're allocating our 1MB bootsector space >>
set 1 boot on
quit

Format it
=========
losetup --find --show --partscan empty.img
<< remember output of the partition >>
lsblk
<< Confirm partition is seen >>
mkfs.fat -s 2 -v -F 16 /dev/loop0p1
<< Should probably use 8+ MB so I don't have to force low sector count >>

Lazy unmount
============
losetup -D
```
