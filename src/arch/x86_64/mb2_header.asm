section .multiboot2
mb2_header_start:
    dd 0xe85250d6
    dd 0
    dd mb2_header_end - mb2_header_start
    dd 0x100000000 - (0xe85250d6 + 0 + (mb2_header_end - mb2_header_start))

    ;dw 7  ; efi services tag
    ;dw 0
    ;dd 8

    ;dw 9  ; efi amd64 tag
    ;dw 0
    ;dd 12
    ;dd 0  ; addr = 0

    dw 0  ; end tag
    dw 0
    dd 8

mb2_header_end:
