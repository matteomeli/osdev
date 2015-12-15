ENTRY(start)

SECTIONS {
    . = 1M;

    .boot :
    {
        KEEP(*(.multiboot2))
    }

    .text ALIGN(0x1000) :
    {
        *(.text .text.*)
    }

    .data ALIGN(0x1000) :
    {
        *(.data .data.*)
    }

    .rodata ALIGN(0x1000) :
    {
        *(.rodata .rodata.*)
    }

    .data.rel.ro ALIGN(0x1000) :
    {
        *(.data.rel.ro.local*) *(.data.rel.ro .data.rel.ro.*)
    }

    .bss ALIGN(0x1000) :
    {
        *(COMMON)
        *(.bss)
    }
}
