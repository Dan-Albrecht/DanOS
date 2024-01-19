ENTRY(_start)

SECTIONS {
    # Must match where the stage1 loader drops this
    . = 0xC800;

    .start : {
        *(.start)
    }
    .text : {
        *(.text .text.*)
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
