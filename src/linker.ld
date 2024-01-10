ENTRY(_entry)
#qemu loader jump to 0x40080000
start_addr = 0xffff000040080000;


SECTIONS
{
    . =  start_addr;
    .text : ALIGN(4k)
    {
        __text_start = .;
        KEEP(*(.text._entry))
        KEEP(*(.text.el1_entry))
        *(.text*)
        __text_end = .;
    } :RX_DATA

    .rodata : ALIGN(4k)  {
        __ro_start = .;
        *(.rodata*)
        __ro_end = .;

     } :RO_DATA


    .data : ALIGN(4k)
    {
       KEEP(*(.data.boot_page_table))
       . = ALIGN(4k);
       KEEP(*(.data.kernel_root))
        __data_start = .;
        *(.data*)
        __data_end = .;

    } :RW_DATA

    .bss : ALIGN(4k)
    {
        __bss_start = .;
        *(.bss*);
        __bss_end = .;
    } :RW_DATA

    .stack : ALIGN(4k)
    {
        __stack_start = .;
        . += 4k * 4;
        __stack_end = .;
    } :RW_DATA

    .symbols : ALIGN(4k){
        __symbols_start = .;
        //200k symbol_table
        . += 200k;
        __symbols_end = .;
    }:RO_DATA

    .heap : ALIGN(4k)
    {
        __heap_start = .;
    }
}
