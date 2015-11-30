global long_mode_start

section .text
bits 64
long_mode_start:
    call setup_SSE

    mov al, 0x0A
    mov dx, 0x3D4
    out dx, al

    mov dx, 0x3D5
    in al, dx

    ; call the rust main
    extern rust_main
    call rust_main
    
.os_returned:
    ; rust main returned, print `OS returned!`
    mov rax, 0x4f724f204f534f4f
    mov [0xb8000], rax
    mov rax, 0x4f724f754f744f65
    mov [0xb8008], rax
    mov rax, 0x4f214f644f654f6e
    mov [0xb8010], rax

    ; Activate cursor
    mov al, 0ah
    mov dx, 3d4h
    out dx, al

    mov dx, 3d5h
    in al, dx   ; cursor start

    mov cl, 1fh
    and al, cl
    mov bl, al  ; copy cursor start & 0xig in bl

    mov al, 0ah
    mov dx, 3d4h
    out dx, al

    mov al, 20h
    not al
    and bl, al

    mov dx, 3d5h
    out dx, al

    ; set it to row 0 column 0
    mov al, 0eh
    mov dx, 3d4h
    out dx, al

    mov al, 00h
    mov dx, 3d5h
    out dx, al
    
    mov al, 0fh
    mov dx, 3d4h
    out dx, al

    mov al, 00h
    mov dx, 3d5h    
    out dx, al

    hlt

; Check for SSE and enable it. If it's not supported throw error "a".
setup_SSE:
    ; check for SSE
    mov rax, 0x1
    cpuid
    test edx, 1<<25
    jz .no_SSE

    ; enable SSE
    mov rax, cr0
    and ax, 0xFFFB      ; clear coprocessor emulation CR0.EM
    or ax, 0x2          ; set coprocessor monitoring  CR0.MP
    mov cr0, rax
    mov rax, cr4
    or ax, 3 << 9       ; set CR4.OSFXSR and CR4.OSXMMEXCPT at the same time
    mov cr4, rax

    ret
.no_SSE:
    mov al, "a"
    jmp error

; Prints `ERROR: ` and the given error code to screen and hangs.
; parameter: error code (in ascii) in al
; We need this new function, as the other one is in now invalid 32 bit code
error:
    mov rbx, 0x4f4f4f524f524f45
    mov [0xb8000], rbx
    mov rbx, 0x4f204f204f3a4f52
    mov [0xb8008], rbx
    mov byte [0xb800e], al
    hlt
    jmp error