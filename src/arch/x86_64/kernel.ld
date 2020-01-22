ENTRY(multiboot2_i386_start)

kernel_stack_size = 1M;

SECTIONS {
  /* virtual offset is 2 MB */
  /* use 2M + SIZEOF_HEADERS combined with ld -n to start linking right
    * after header so that ld won't waste a whole page on padding */
  . = 2M + SIZEOF_HEADERS;

  /* ensure that the multiboot header comes as early as possible */
  .multiboot2 : {
    KEEP(*(.multiboot2))
  }

  .boot.text : {
    *(.boot.text .boot.text.*)
  }
  .boot.rodata : {
    *(.boot.rodata .boot.rodata.*)
  }
  .boot.data : {
    *(.boot.data .boot.data.*)
  }
  .boot.bss : {
    *(.boot.bss .boot.bss.*)
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
  .bss : {
    *(.bss .bss.*)
  }

  .stack_sizes (INFO) : {
    KEEP(*(.stack_sizes .stack_sizes.*))
  }

  kernel_stack_bottom = .;
  . += kernel_stack_size;
  . = ALIGN(16);
  kernel_stack_top = .;
}
