
MULTIBOOT_MAGIC     equ 0xe85250d6
ARCHITECTURE_FLAG   equ 0

section .multiboot2
align 8
header_start:
    dd MULTIBOOT_MAGIC              ; magic number (multiboot 2)
    dd ARCHITECTURE_FLAG            ; multiboot flag for architecture 0 (protected mode i386)
    dd header_end - header_start    ; header length
    ; calculate the checksum (magic number + checksum + flags + header length should equal 0)
    dd 0x100000000 - (MULTIBOOT_MAGIC + ARCHITECTURE_FLAG + (header_end - header_start))                                
    
    ; insert optional multiboot tags

    ; required end tag
    dw 0    ; type
    dw 0    ; flags
    dd 8    ; size
header_end: