ENTRY(start)

SECTIONS {
    . = 1M;

    .boot :
    {
         KEEP(*(.header))
    }

    .text :
    {
        *(.text)
    }
}