ENTRY(_start)
start_addr = 0x00400000;

SECTIONS
{
    . =  start_addr;
    .text : ALIGN(4k)
    {
        __text_start = .;
        KEEP(*(.text._start))
        *(.text*)
        __text_end = .;
    }

    .rodata : ALIGN(4k)  {
        __ro_start = .;
        *(.rodata*)
        __ro_end = .;

     }


    .data : ALIGN(4k)
    {
       . = ALIGN(4k);
        __data_start = .;
        *(.data*)
        __data_end = .;

    }

    .bss : ALIGN(4k)
    {
        __bss_start = .;
        *(.bss*);
        __bss_end = .;
    }
}
