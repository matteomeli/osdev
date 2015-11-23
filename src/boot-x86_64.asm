global start

section .text
bits 32
start:
    mov esp, kernel_stack_top           ; point esp to the start of the stack (end of memory, stack grows downwards)

    ; call the rust main
    extern rust_main
    call rust_main

    mov dword [0xb8000], 0x2f4b2f4f     ; print OK to screen
    hlt

section .bss:
align 4
kernel_stack_bottom:
    resb 64                             ; reserve 64 bytes for the kernel stack
kernel_stack_top: