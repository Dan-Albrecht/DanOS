use core::arch::asm;

pub unsafe fn outB(port: u16, value: u8) { unsafe {
    asm!(
        "out dx, al",
        in("dx") port,
        in("al") value
    )
}}

pub unsafe fn inB(port: u16) -> u8 { unsafe {
    let result: u8;
    asm!(
        "in al, dx",
        out("al") result,
        in("dx") port
    );

    result
}}
