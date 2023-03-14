ENTRY(_entry)
BASE_ADDRESS = 0xffff000040080000;

SECTIONS
{
    . = BASE_ADDRESS;
    start_kernel = .;

    .text : {
        start_text = .;
        *(.text.entry)
        *(.text .text.*)
        . = ALIGN(4K);
        end_text = .;
    }

    .rodata : {
        start_ro_data = .;
        . = ALIGN(4K);
        end_ro_data = .;
    }

    .data : {
        start_data = .;
        . = ALIGN(4K);
        end_data = .;
    }

    .bss : {
        start_bss = .;
        . = ALIGN(4K);
        end_bss = .;
    }
    .stack : {
        boot_stack = .;
        . += 4k * 4;
        . = ALIGN(4K);
        boot_stack_top = .;
    }
    end_kernel = .;
}
