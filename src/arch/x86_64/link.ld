ENTRY(start)                    /* the name of the entry label */

SECTIONS {
    . = 1M;                     /* the code should be loaded at 1 MB */

    .boot :
    {
        *(.multiboot_header)    /* ensure header is at the beginning */
    }

    .text :
    {
        *(.text)                /* all text sections from all files */
    }
}