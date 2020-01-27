global multiboot2_i386_start
extern x86_64_start
extern kernel_stack_top

kernel_load_base equ 0x400000
kernel_base equ 0xffffff8000000000

section .boot.text progbits alloc exec nowrite align=16
bits 32
multiboot2_i386_start:
    cmp eax, 0x36d76289  ; multiboot2 signature
    jne .die

    mov [multiboot2_info], ebx  ; save multiboot2 info pointer

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

	; pml4[0] -> pdpt.lo
    lea edi, [pdpt.lo]
    or edi, 0x3  ; P, W, S
    mov [pml4], edi  ; lower 32 bits of entry 0; no need to set up upper bits

    ; pml4[511] -> pdpt.hi
    lea edi, [pdpt.hi]
    or edi, 0x3  ; P, W, S
    mov [pml4+511*8], edi

.setup_pdpt:
    lea edi, [pdpt]
    xor eax, eax
    mov ecx, pdpt.end-pdpt
    rep stosb

.setup_pdpte_lo:
    ; identity map first GB
    mov dword [pdpt.lo], 0x183  ; addr=0; P, W, PS, G

.setup_pdpte_hi:
    lea edi, [pd.hi]
    or edi, 0x3  ; P, W, S
    mov [pdpt.hi], edi

.setup_pd:
    lea edi, [pd]
    xor eax, eax
    mov ecx, pd.end-pd
    rep stosb

.setup_pde_hi:
    ; map 8 MB starting from kernel_load_base into kernel_base
    mov dword [pd.hi], 0x400183  ; addr = 0x400000; P, W, PS, G
    mov dword [pd.hi+8], 0x600183  ; addr = 0x400000; P, W, PS, G
    mov dword [pd.hi+16], 0x800183  ; addr = 0x400000; P, W, PS, G
    mov dword [pd.hi+24], 0xa00183  ; addr = 0x400000; P, W, PS, G

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
    ; set up 64-bit GDT to enable 64-bit mode
    lgdt [gdt64.ptr]
    jmp gdt64.code:mb2_lm_trampoline

.die:
    lidt [noidt]  ; causes a triple fault
    int 3

section .boot.text.64 progbits alloc exec nowrite align=16
bits 64
mb2_lm_trampoline:
    mov rsp, kernel_stack_top
    ;add rsp, kernel_base
    mov rbp, rsp
    ;add eax, kernel_base
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
pd:
.hi:
    resb 4096
.end:
multiboot2_info:
    resq 1

section .boot.rodata progbits alloc noexec nowrite align=4
noidt:
    dw 0
    dd 0

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
