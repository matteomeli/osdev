;;; Interrupts support

VGA_FB_BASE equ 0xB8000

global dummy_interrupt_handler
global interrupt_handlers

extern rust_interrupt_handler

section .text
bits 64

;;; This macro generates functions for non-error interrupts.
;;;
;;; For consistency, push a 0 error code along 
;;; to the called interrupt handler.
%macro no_error_code_interrupt_handler 1
interrupt_handler_%1:
    push qword 0                    ; Push 0 as error code
    push qword %1                   ; Push interrupt ID
    jmp interrupt_common_handler    ; Jump to the common handler function
%endmacro

;;; This macro generates functions for error coded interrupts.
%macro error_code_interrupt_handler 1
interrupt_handler_%1:
    ;; There's already a qword error code pushed here.
    ;; A 0 dword for padding and the error code.
    ;; This qword will need to be popped before returning from the common handler.
    push qword %1                   ; Push interrupt ID
    jmp interrupt_common_handler    ; Jump to the common handler function
%endmacro

;;; Registers to save are listed here:
;;; http://wiki.osdev.org/System_V_ABI,
;;; https://www.cs.cmu.edu/~fp/courses/15213-s07/misc/asm64-handout.pdf
;;;
;;; Callee-saved register are skipped, as responsability of Rust compiler.
;;; Don't save any floating point register as it's slow (512 bytes). Support
;;; for FPU is stripped out of the kernel through recompiling Rust libcore
;;; without FP support.
%macro push_caller_saved_registers 0
    push rax
    push rcx
    push rdx
    push r8
    push r9
    push r10
    push r11
    ;; These are calle-saved in i386 but caller-saved in x86_64
    push rdi
    push rsi
%endmacro

;;; Pop registers in reverse order
%macro pop_caller_saved_registers 0
    pop rsi
    pop rdi
    pop r11
    pop r10
    pop r9
    pop r8
    pop rdx
    pop rcx
    pop rax
%endmacro

;;; All the interrupt handlers end up here, just a wrapper into a Rust function.
interrupt_common_handler:
    ;; Push on the stack caller-saved registers
    push_caller_saved_registers

    ;; Pass pointer to interrupt data (error code and interrupt ID)
    mov rdi, rsp    ; rdi register contains 1st argument for function calls
    ;; Call rust
    call rust_interrupt_handler

    ;; Pop the previously saved register
    pop_caller_saved_registers

    ;; Restore ESP
    add rsp, 16     ; Clean error code and interrupt ID placed here before

    iretq

;;; A dummy handler that just prints INT! to the VGA frame buffer
dummy_interrupt_handler:
    push_caller_saved_registers

    mov rax, 0x4f214f544f4e4f49
    mov qword [VGA_FB_BASE], rax

    pop_caller_saved_registers

    iretq

;;; Generate list of interrupts. The list is taken from here:
;;; http://developer.amd.com/wordpress/media/2012/10/24593_APM_v21.pdf,
;;; https://en.wikipedia.org/wiki/Interrupt_descriptor_table,
;;; http://www.intel.com/Assets/en_US/PDF/manual/253668.pdf, Chapter 6
;;;
;;; 0x00 (Integer Division by Zero Exception)
no_error_code_interrupt_handler 0
;;; 0x01 (Debug Exception)
no_error_code_interrupt_handler 1
;;; 0x02 (Non Maskable Input)
no_error_code_interrupt_handler 2
;;; 0x03 (Breakpoint Exception INT 3)
no_error_code_interrupt_handler 3
;;; 0x04 (Overflow Exception INTO instruction)
no_error_code_interrupt_handler 4
;;; 0x05 (Bound-Range Exception BOUND instruction)
no_error_code_interrupt_handler 5
;;; 0x06 (Invalid-Opcode Exception)
no_error_code_interrupt_handler 6
;;; 0x07 (Device-Not-Available Exception)
no_error_code_interrupt_handler 7
;;; 0x08 (Double Fault Exception)
error_code_interrupt_handler 8
;;; 0x09 (Coprocessor-Segment-Overrun Exception reserved in AMD64)
;;; 0x10 (Invalid TSS Exception)
error_code_interrupt_handler 10
;;; 0x11 (Segment-Not_Present Exception)
error_code_interrupt_handler 11
;;; 0x12 (Stack Exception)
error_code_interrupt_handler 12
;;; 0x13 (General-Protection Exception)
error_code_interrupt_handler 13
;;; 0x14 (Page-Fault Exception)
error_code_interrupt_handler 14
;;; 0x15 (Reserved)
;;; 0x16 (x87 Floating-Point Exception)
no_error_code_interrupt_handler 16
;;; 0x17 (Alignment-Check Exception)
error_code_interrupt_handler 17
;;; 0x18 (Machine-Check Exception)
no_error_code_interrupt_handler 18
;;; 0x19 (SIMD Floating-Point Exception)
no_error_code_interrupt_handler 19

;;; Generate all the 32-255 Hardware Interrupts (PIC managed)
no_error_code_interrupt_handler 32
%assign i 33
%rep 224
no_error_code_interrupt_handler i
%assign i i+1
%endrep

section .rodata
interrupt_handlers:
    dq interrupt_handler_0
    dq interrupt_handler_1
    dq interrupt_handler_2
    dq interrupt_handler_3
    dq interrupt_handler_4
    dq interrupt_handler_5
    dq interrupt_handler_6
    dq interrupt_handler_7
    dq interrupt_handler_8
    dq 0
    dq interrupt_handler_10
    dq interrupt_handler_11
    dq interrupt_handler_12
    dq interrupt_handler_13
    dq interrupt_handler_14
    dq 0
    dq interrupt_handler_16
    dq interrupt_handler_17
    dq interrupt_handler_18
    dq interrupt_handler_19
    dq 0
    dq 0
    dq 0
    dq 0
    dq 0
    dq 0
    dq 0
    dq 0
    dq 0
    dq 0
    dq 0                        ; interrupt handler 30
    dq 0
%assign i 32
%rep 224
    dq interrupt_handler_%+i
%assign i i+1
%endrep
