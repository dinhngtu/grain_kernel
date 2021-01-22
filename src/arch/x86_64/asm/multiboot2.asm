default rel

global multiboot2_i386_start
extern x86_64_start
extern kernel_stack_top

serial_port equ 0x40a0

section .boot.text progbits alloc exec nowrite align=16
bits 32
multiboot2_i386_start:
    cmp eax, 0x36d76289  ; multiboot2 signature
    jne .die.real

    test ebx, ebx
    jz .die.real
    mov [multiboot2_info], ebx  ; save multiboot2 info pointer
    lea ebx, [.inita]
    jmp serial.init

.inita:
    lea ebx, [.check_long_mode]
    lea esi, [strtbl.hello]
    jmp serial.write

.check_long_mode:
    mov eax, 0x80000000
    cpuid
    cmp eax, 0x80000001  ; extended cpuid leaves?
    jb .die

    mov eax, 0x80000001
    cpuid
    test edx, 1<<29  ; long mode?
    jz .die

; paging already disabled
.setup_pml4:
    lea edi, [pml4]
    xor eax, eax
    mov ecx, 4096
    rep stosb ; loop store al to edi while cx <> 0

	; pml4[0] -> pdpt.lo
    lea edi, [pdpt.lo]
    or edi, 0x3  ; P, W, S
    mov [pml4], edi  ; lower 32 bits of entry 0; no need to set up upper bits

    ; pml4[256] -> pdpt.hi
    lea edi, [pdpt.hi]
    or edi, 0x3  ; P, W, S
    mov [pml4+256*8], edi

.setup_pdpt:
    lea edi, [pdpt]
    xor eax, eax
    lea ecx, [pdpt.end-pdpt]
    rep stosb

    lea edi, [pdpt.lo]
    xor eax, eax
.setup_pdpte_lo:
    mov edx, eax
    shl edx, 30  ; address lower bits
    or edx, 0x183  ; P, W, PS, G
    mov [edi+eax*8], edx

    mov edx, eax
    shr edx, 2  ; address upper bits
    mov [edi+eax*8+4], edx

    inc eax
    cmp eax, 512
    jb .setup_pdpte_lo

    lea edi, [pdpt.hi]
    xor eax, eax
.setup_pdpte_hi:
    ; loop for mapping physical memory into lowest 512GB of upper half
    mov edx, eax
    shl edx, 30  ; address lower bits
    or edx, 0x183  ; P, W, PS, G
    mov [edi+eax*8], edx

    mov edx, eax
    shr edx, 2  ; address upper bits
    mov [edi+eax*8+4], edx

    inc eax
    cmp eax, 512
    jb .setup_pdpte_hi

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
    or eax, 1<<8  ; EFER.LME
    wrmsr
    ; enable PG
    mov eax, cr0
    or eax, 1<<31
    mov cr0, eax
    ; final check
    mov ecx, 0xc0000080  ; EFER
    rdmsr
    test eax, 1<<10  ; EFER.LMA
    jz .die

    lea ebx, [.jump]
    lea esi, [strtbl.ajh]
    jmp serial.write

.jump:
    ; set up 64-bit GDT to enable 64-bit mode
    lgdt [gdt64.ptr]
    jmp gdt64.code:mb2_lm_trampoline

.die:
    lea ebx, [.die.real]
    lea esi, [strtbl.die]
    jmp serial.write
.die.real:
    lidt [noidt]  ; causes a triple fault
    int 3

serial:
.init:
    mov dx, serial_port+3
    mov al, 0x03
    out dx, al
    mov dx, serial_port+1
    xor eax, eax
    out dx, al
    mov dx, serial_port+2
    out dx, al
    mov al, 0x03
    mov dx, serial_port+4
    out dx, al
    jmp ebx

; esi: string pointer
.write:
    mov cl, byte [esi]
    test cl, cl
    jnz .writea
    jmp ebx
.writea:
    ; wait to write
    mov dx, serial_port+5
    in al, dx
    test al, 0x20
    jz .writea
    ; write
    mov dx, serial_port
    mov al, cl
    out dx, al
.writec:
    ; wait for write
    mov dx, serial_port+5
    in al, dx
    test al, 0x40
    jz .writec
    ; loop
    inc esi
    jmp .write

section .boot.text.64 progbits alloc exec nowrite align=16
bits 64
mb2_lm_trampoline:
    ; be careful with symbols that live in upper half
    mov rsp, kernel_stack_top
    mov rbp, rsp
    mov edi, [multiboot2_info]
    mov rax, x86_64_start
    jmp rax

section .boot.bss nobits alloc noexec write align=4
alignb 4096
pml4:
    resb 4096
pdpt:
.lo:
    resb 4096
.hi:
    resb 4096
.end:
multiboot2_info:
    resq 1

section .boot.rodata progbits alloc noexec nowrite align=4
noidt:
    dw 0
    dd 0

strtbl:
.hello:
    db `hello from bootloader\n\0`
.ajh:
    db `jumping to x64\n\0`
.die:
    db `death\n\0`

; minimal required gdt for real long mode
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
