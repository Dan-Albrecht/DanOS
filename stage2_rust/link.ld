ENTRY(DanMain)

SECTIONS {
    . = STAGE2_RUST_LOAD_TARGET;

    .text : {
        *(.text.DanMain);
        *(.text*);
    }
    .rodata : {
        *(.rodata .rodata.*)
    }
    .data : {
        *(.data .data.*)
    }
    .bss : {
        *(.bss .bss.*)
    }
}
