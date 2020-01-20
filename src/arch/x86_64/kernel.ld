ENTRY(multiboot2_i386_start)

kernel_stack_size = 1M;

SECTIONS {

    /* somehow needed for grub to recognize the multiboot header */
    . = 2M + SIZEOF_HEADERS;

    .boot : {
        /* ensure that the multiboot header is at the beginning */
        KEEP(*(.multiboot2))

        /* ensure that the address (tss-0x200000) fits in 16bit */
        *(.tss)
    }

    .text : {
      *(.text .text.*)
    }

    .rodata : {
      *(.rodata .rodata.*)
    }

    .data.rel.ro : {
      *(.data.rel.ro .data.rel.ro.*)
    }

    .data : {
      *(.data .data.*)
    }

    kernel_stack_bottom = .;
    . += kernel_stack_size;
    . = ALIGN(16);
    kernel_stack_top = .;
}
