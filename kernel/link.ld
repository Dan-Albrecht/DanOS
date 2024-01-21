ENTRY(DanMain)

SECTIONS {
    . = KERNEL32_LOAD_TARGET;

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
