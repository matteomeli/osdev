global start
global gdt64_code_offset

extern long_mode_start

section .text
bits 32
start:
    mov esp, kernel_stack_top           ; Set esp to the start of the stack
                                        ; (top of stack memory, stack grows downwards)
    mov edi, ebx                        ; Move Multiboot info pointer to edi

    call test_multiboot
    call test_cpuid
    call test_long_mode

    call setup_page_tables
    call enable_paging
    call setup_SSE

    ; load the 64bit GDT
    lgdt [gdt64.pointer]

    ; update selectors
    mov ax, gdt64.data
    mov ss, ax  ; stack selector
    mov ds, ax  ; data selector
    mov es, ax  ; extra selector

    ;; No way of setting the cs (code selector) manually.
    ;; A far jump to 64 bit code is needed
    jmp gdt64.code:long_mode_start

test_multiboot:
    cmp eax, 0x36d76289
    jne .no_multiboot
    ret
.no_multiboot:
    mov al, "0"
    jmp error

test_cpuid:
    pushfd               ; Store the FLAGS-register.
    pop eax              ; Restore the A-register.
    mov ecx, eax         ; Set the C-register to the A-register.
    xor eax, 1 << 21     ; Flip the ID-bit, which is bit 21.
    push eax             ; Store the A-register.
    popfd                ; Restore the FLAGS-register.
    pushfd               ; Store the FLAGS-register.
    pop eax              ; Restore the A-register.
    push ecx             ; Store the C-register.
    popfd                ; Restore the FLAGS-register.
    xor eax, ecx         ; Do a XOR-operation on the A-register and the C-register.
    jz .no_cpuid         ; The zero flag is set, no CPUID.
    ret                  ; CPUID is available for use.
.no_cpuid:
    mov al, "1"
    jmp error

test_long_mode:
    mov eax, 0x80000000    ; Set the A-register to 0x80000000.
    cpuid                  ; CPU identification.
    cmp eax, 0x80000001    ; Compare the A-register with 0x80000001.
    jb .no_long_mode       ; It is less, there is no long mode.
    mov eax, 0x80000001    ; Set the A-register to 0x80000001.
    cpuid                  ; CPU identification.
    test edx, 1 << 29      ; Test if the LM-bit, which is bit 29, is set in the D-register.
    jz .no_long_mode       ; They aren't, there is no long mode.
    ret
.no_long_mode:
    mov al, "2"
    jmp error

; Setup Paging. 
; In Long Mode, x86 use a 4 level page table with page size of 4096
; Each page table contains 512 entries, each entry is 8 bytes (512*8=4096)
; Each 64bit virtual address is used to index the the tables:
;   1) First 9 bits entry (2^9 = 512 entries) in PML4 table (base address in CR3 register)
;   2) Second 9 bits offset in PDP table
;   3) Third 9 bits offset in PD table
;   4) Fourth 9 bits offset in PT (Page Table) table
;   5) Last 12 bits to offset 4K physical memory page (2^12 = 4096)
;   NM) Bits 48-63 rest unused, actual copies of bit 47.
; Each entry in the tables contains the page aligned 52bit physical address of
; the next table, ORed in with some bit flags (present 0, writable 1, etc.).
; Finally, to identity map the first gigabytes of memory, use the following mapping:
; 1 PML4 -> 1 PDP -> 512 2MiB PD tables
setup_page_tables:
    ; recursive map P4
    mov eax, p4_table
    or eax, 0b11 ; present + writable
    mov [p4_table + 511 * 8], eax

    ; map first P4 entry to P3
    mov eax, p3_table
    or eax, 0b11            ; Set present and writable flags
    mov [p4_table], eax     ; p4_table is a address. [addr] deferences the memory at the addr

    ; map first P3 entry to P2
    mov eax, p2_table
    or eax, 0b11            ; Set present and writable flags
    mov [p3_table], eax

    ; map each P2 entry to a 2MiB page
    mov ecx, 0              ; counter variable
.map_p2_table:
    ; map ecx-th P2 entry to a huge page that start at address ecx*2MiB
    mov eax, 0x200000               ; 2 MiB
    mul ecx                         ; Multiply what's in eax with ecx and store in eax (start address of ecx-th page)
    or eax, 0b10000011              ; Set present, writable and huge page (in P2 means 2 MiB pages) flags
    mov [p2_table + ecx * 8], eax   ; Set each ecx-th entry of P2 to the ecx-th page

    inc ecx                         ; Increment counter
    cmp ecx, 512                    ; if ecx == 512, we're done
    jne .map_p2_table               ; else loop again

    ret

enable_paging:
    ; load P4 adress into cr3 register
    mov eax, p4_table
    mov cr3, eax

    ; enable PAE flag in cr4 (Physical Adrees Extension)
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    ; set the long mode bit in the EFER MSR (model specific register)
    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    ; enable paging in the cr0 register
    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax

    ret

; Check for SSE and enable it. If it's not supported throw error "a".
setup_SSE:
    ; check for SSE
    mov eax, 0x1
    cpuid
    test edx, 1<<25
    jz .no_SSE

    ; enable SSE
    mov eax, cr0
    and ax, 0xFFFB      ; clear coprocessor emulation CR0.EM
    or ax, 0x2          ; set coprocessor monitoring  CR0.MP
    mov cr0, eax
    mov eax, cr4
    or ax, 3 << 9       ; set CR4.OSFXSR and CR4.OSXMMEXCPT at the same time
    mov cr4, eax

    ret
.no_SSE:
    mov al, "a"
    jmp error

; Prints `ERR: ` and the given error code to screen and hangs.
; parameter: error code (in ascii) in al
error:
    mov dword [0xb8000], 0x4f524f45
    mov dword [0xb8004], 0x4f3a4f52
    mov dword [0xb8008], 0x4f204f20
    mov byte  [0xb800a], al
    hlt

section .bss
align 4096
p4_table:                   ; Page-Map Level-4 Table (PML4) or P4
    resb 4096
p3_table:                   ; Page-Directory Pointer Table (PDP) or P3
    resb 4096
p2_table:                   ; Page-Directory Table (PD) or P2
    resb 4096
;;; Reserve space for the kernel stack.
kernel_stack_bottom:
    resb 4096 * 3           ; Reserve 4096 * 3 bytes for the kernel stack
kernel_stack_top:

section .rodata
;;; Global Description Table. Used to describe available segments.
gdt64:
    dq 0                                                    ; zero entry
.code: equ $ - gdt64
    dq (1<<44) | (1<<47) | (1<<41) | (1<<43) | (1<<53)      ; code segment
.data: equ $ - gdt64
    dq (1<<44) | (1<<47) | (1<<41)                          ; data segment
.pointer:
    dw $ - gdt64 - 1
    dq gdt64

;;; Export code selector so Rust can read it.
gdt64_code_offset:
    dw gdt64.code
