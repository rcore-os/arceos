OUTPUT_ARCH(%ARCH%)

BASE_ADDRESS = %KERNEL_BASE%;

ENTRY(_start)
SECTIONS
{
    . = BASE_ADDRESS;
    skernel = .;

    .text : ALIGN(4K) {
        stext = .;
        *(.text.boot)
        *(.text .text.*)
        . = ALIGN(4K);
        etext = .;
    }

    .rodata : ALIGN(4K) {
        srodata = .;
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
        . = ALIGN(4K);
        erodata = .;
    }

    .data : ALIGN(4K) {
        sdata = .;
        *(.data.boot_page_table)
        . = ALIGN(4K);
        *(.data .data.*)
        *(.sdata .sdata.*)
        *(.got .got.*)
        . = ALIGN(4K);
        edata = .;
    }

    .bss : ALIGN(4K) {
        boot_stack = .;
        *(.bss.stack)
        . = ALIGN(4K);
        boot_stack_top = .;

        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        . = ALIGN(4K);
        ebss = .;
    }

    ekernel = .;

    /DISCARD/ : {
        *(.eh_frame) *(.eh_frame_hdr)
    }
}
