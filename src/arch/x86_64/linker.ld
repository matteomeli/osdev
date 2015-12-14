ENTRY(start)

SECTIONS {
    . = 1M;

    .boot :
    {
        KEEP(*(.multiboot2))
    }

    .text :
    {
        *(.text)
    }
}