ENTRY(DanMain)

SECTIONS {
    . = KERNEL64_LOAD_TARGET;

    .text : {
        *(.text.DanMain);
        *(.text*);
        *(.text.Stupid);
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
    .eh_frame : {
        *(.eh_frame .eh_frame.*)
    }
    .eh_frame_hdr : {
        *(.eh_frame_hdr .eh_frame_hdr.*)
    }
}
