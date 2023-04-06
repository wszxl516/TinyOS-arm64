ENTRY(_entry);
 
base_addr = 0x40100000;
	
SECTIONS {
    . = base_addr;
	PROVIDE(text_start = .);
	.text : ALIGN(4K) {
		*(.init);
		*(.text);
        . = ALIGN(4K);
	}

	.bss : ALIGN(4K) {
		PROVIDE(bss_start = .);
		*(.bss);
		. += 0x1000;
		PROVIDE(bss_end = .);
	}

	.rodata : ALIGN(4K) {
		*(.rodata);
	}

	.data : ALIGN(4K) {
		*(.data);
	}

	.device : ALIGN(4k) {
		PROVIDE(device_start = .);
		PROVIDE(device_end = .);
	}
	.pagetable : ALIGN(4k) {
		. = ALIGN(0x1000);
		PROVIDE(page_table_start = .);
	}
	.frame : ALIGN(4k) {
		. = ALIGN(0x1000);
		PROVIDE(frame_start = .);
		. += 0x1000 * 1024;
		PROVIDE(frame_end = .);
	}
	.stack : ALIGN(4k) {
		. += 0x1000;
		PROVIDE(stack_bottom = .);
		. += 0x1000 * 4;
		PROVIDE(stack_top = .);
		. += 0x1000;
	}

	.user_stack : ALIGN(4k) {
		. += 0x1000;
		PROVIDE(user_stack_bottom = .);
		. += 0x1000 * 4;
		PROVIDE(user_stack_top = .);
		. += 0x1000;
	}

	PROVIDE(heap_start = .);
}
