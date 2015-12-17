;;; Multiboot 2 header 

MB2_MAGIC_NUMBER    equ 0xe85250d6
MB2_ARCH_FLAG       equ 0x0
MB2_HEADER_LENGTH   equ mb2_header_end - mb2_header_start 

section .multiboot2
mb2_header_start:
    dd MB2_MAGIC_NUMBER      ; magic number (multiboot 2)
    dd MB2_ARCH_FLAG        ; multiboot flag for architecture 0 (protected mode i386)
    dd MB2_HEADER_LENGTH    ; header length
    ; calculate the checksum (magic number + checksum + flags + header length should equal 0)
    dd 0x100000000 - (MB2_MAGIC_NUMBER + MB2_ARCH_FLAG + (mb2_header_end - mb2_header_start))                                
    
    ; insert optional multiboot tags

    ; required end tag
    dw 0    ; type
    dw 0    ; flags
    dd 8    ; size
mb2_header_end:
