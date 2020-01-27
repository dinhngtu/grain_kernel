section .multiboot2

;Offset , Type , Field         , Name     , Note
;0      , u32i , magic         , required
;4      , u32  , architecture  , required
;8      , u32  , header_length , required
;12     , u32  , checksum      , required
;16-XX  , tags , required

align 8
mb2_header_start:
    dd 0xe85250d6
    dd 0 ; protected mode code grub will automatocally do the transition from 16bits to 32bits/A20 line
    dd mb2_header_end - mb2_header_start
    dd 0x100000000 - (0xe85250d6 + 0 + (mb2_header_end - mb2_header_start))

;        Tags
;        +-------------------+
;u16     | type              |
;u16     | flags             |
;u32     | size              |
;        +-------------------+

    align 8
; required, grub failed to load if end tag not present
    dw 0  ; end tag
    dw 0
    dd 8
mb2_header_end:
