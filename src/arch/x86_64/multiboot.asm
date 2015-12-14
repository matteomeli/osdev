MB_MAGIC        equ 0xe85250d6
MB_ARCH_FLAG    equ 0
MB_HEADER_LEN   equ mb2_header_end - mb2_header_start

section .multiboot2
align 8
mb2_header_start:
    dd MB_MAGIC             ; magic number (multiboot 2)
    dd MB_ARCH_FLAG         ; multiboot flag for architecture 0 (protected mode i386)
    dd MB_HEADER_LEN        ; header length
    ; calculate the checksum (magic number + checksum + flags + header length should equal 0)
    dd 0x100000000 - (MB_MAGIC + MB_ARCH_FLAG + MB_HEADER_LEN)                                
    
    ; insert optional multiboot tags

    ; required end tag
    dw 0    ; type
    dw 0    ; flags
    dd 8    ; size
mb2_header_end:
