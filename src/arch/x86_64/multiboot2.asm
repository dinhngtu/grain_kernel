cmp eax, 0x36d76289; This file setup the long mode and plm4

global multiboot2_i386_start
extern x86_64_start
extern kernel_stack_top

section .boot.text progbits alloc exec nowrite align=16
bits 32
multiboot2_i386_start:

    cmp eax, 0x36d76289  ; multiboot2 signature
    jne .die

    mov [multiboot2_info], ebx  ; save multiboot2 info pointer

.check_cpuid:
 ; Copy FLAGS in to EAX via stack
    pushfd
    pop eax
    ; Copy to ECX as well for comparing later on
    mov ecx, eax
    ; Flip the ID bit
    xor eax, 1 << 21
    ; Copy EAX to FLAGS via the stack
    push eax
    popfd
    ; Copy FLAGS back to EAX (with the flipped bit if CPUID is supported)
    pushfd
    pop eax
    ; Restore FLAGS from the old version stored in ECX (i.e. flipping the ID bit
    ; back if it was ever flipped).
    push ecx
    popfd
    ; Compare EAX and ECX. If they are equal then that means the bit wasn't
    ; flipped, and CPUID isn't supported.
    xor eax, ecx
    jz .die

.check_long_mode:
    mov eax, 0x80000000
    cpuid
    cmp eax, 0x80000001  ; extended cpuid leaves?
    jb .die

    mov eax, 0x80000001
    cpuid
    test edx, 1 << 29  ; long mode?
    jz .die

; paging already disabled
.setup_pml4:
    lea edi, [pml4]
    xor eax, eax
    mov ecx, 4096
    rep stosb ; loop store al to edi while cx <>  0

	;plm4[0] -> pdpt
    lea edi, [pdpt]
    or edi, 0x3  ; P, W, S
    mov [pml4], edi  ; lower 32 bits of entry 0; no need to set up upper bits

.setup_pdpt:
    lea edi, [pdpt]
    xor eax, eax
    mov ecx, 4096
    rep stosb

    lea edi, [pdpt]
    ;xor eax, eax
.setup_pdpte:
    ; loop for identity mapping 512 GB
    mov edx, eax
    shl edx, 30  ; address lower bits
    or edx, 0x183  ; P, W, PS, G
    mov [edi+eax*8], edx

    mov edx, eax
    shr edx, 2  ; address upper bits
    mov [edi+eax*8+4], edx

    inc eax
    cmp eax, 512
    jb .setup_pdpte

.setup_lm:
    lea edi, [pml4]
    mov cr3, edi
    ; configure CR4
    mov eax, cr4
    or eax, 0x30  ; CR4.PSE, CR4.PAE
    mov cr4, eax
    ; configure EFER
    mov ecx, 0xc0000080  ; EFER
    rdmsr
    or eax, 0x900  ; EFER.LME, EFER.NXE
    wrmsr
    ; enable PG
    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax
    ; final check
    mov ecx, 0xc0000080  ; EFER
    rdmsr
    test eax, 1 << 10  ; EFER.LMA
    jz .die
    ; set up 64-bit GDT to enable 64-bit mode
    mov edi, [multiboot2_info]
    lea esp, [kernel_stack_top]
    lea ebp, [kernel_stack_top]
    push .die
    lgdt [gdt64.ptr]
    jmp gdt64.code:x86_64_start

.die:
    lidt [noidt]  ; causes a triple fault
    int 3

section .boot.bss nobits alloc noexec write align=4
alignb 4096
pml4:
    resb 4096
pdpt:
    resb 4096
multiboot2_info:
    resq 1

section .boot.rodata progbits alloc noexec nowrite align=4
noidt:
    dw 0
    dd 0

;minimal required gdt for real long mode
gdt64:
.null: equ $-gdt64
    dq 0
.code: equ $-gdt64
    dw 0xffff  ; limit low
    dw 0  ; base low
    db 0  ; base mid
    db 0x9a  ; type=RX, code, present
    db 0xaf  ; limit high, long, granularity=4K
    db 0  ; base high
.data: equ $-gdt64
    dw 0  ; limit low
    dw 0  ; base low
    db 0  ; base mid
    db 0x92  ; type=RW, data, present
    db 0x20  ; limit high, long, granularity=byte
    db 0  ; base high
.ptr:
    dw $-gdt64-1  ; limit
    dq gdt64  ; base
