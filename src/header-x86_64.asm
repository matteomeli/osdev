section .header
header_start:
    dd 0xe85250d6                   ; magic number (multiboot 2)
    dd 0x0                          ; multiboot flag for architecture 0 (protected mode i386)
    dd header_end - header_start    ; header length
    ; calculate the checksum (magic number + checksum + flags + header length should equal 0)
    dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))                                
    
    ; insert optional multiboot tags

    ; required end tag
    dw 0    ; type
    dw 0    ; flags
    dd 8    ; size
header_end: