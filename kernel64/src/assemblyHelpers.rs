use core::arch::asm;

pub fn getCR2() -> u64 {
    let cr2Value : u64;

    unsafe {
        asm!(
            "mov rax, cr2",
            out("rax") cr2Value,
        );
    }

    cr2Value
}
