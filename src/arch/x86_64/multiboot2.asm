section .multiboot2
mb2_start:
    dd 0xe85250d6
    dd 0
    dd mb2_end - mb2_start
    dd 0x100000000 - (0xe85250d6 + 0 + (mb2_end - mb2_start))
mb2_end:
