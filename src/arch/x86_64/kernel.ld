ENTRY(multiboot2_i386_start)

kernel_load_base = 0x400000;
/* one PML4E */
kernel_base = 0xffffff8000000000;
kernel_load_offset = kernel_base - kernel_load_base;
kernel_stack_size = 16K;

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

  . = kernel_base;

  .text : AT(ADDR(.text) - kernel_load_offset) ALIGN(0x1000) {
    *(.text .text.*)
  }
  .rodata : AT(ADDR(.rodata) - kernel_load_offset) ALIGN(0x1000) {
    *(.rodata .rodata.*)
  }
  .data : AT(ADDR(.data) - kernel_load_offset) ALIGN(0x1000) {
    *(.data .data.*)
  }
  .bss : AT(ADDR(.bss) - kernel_load_offset) ALIGN(0x1000) {
    *(.bss .bss.*)
  }

  kernel_stack_bottom = .;
  . += kernel_stack_size;
  . = ALIGN(16);
  kernel_stack_top = .;
}
