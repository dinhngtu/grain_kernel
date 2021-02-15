section .multiboot2 align=8

align 8
mb2_header:
.start:
    dd 0xe85250d6  ; magic
    dd 0  ; protected mode
    dd .end-.start  ; header length
    dd 0x100000000 - (0xe85250d6 + 0 + (.end-.start))  ; csum

    align 8
; required, grub failed to load if end tag not present
    dw 0  ; end tag
    dw 0
    dd 8
.end:
