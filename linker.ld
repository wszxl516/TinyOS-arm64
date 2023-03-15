ENTRY(_entry);
 
base_addr = 0x40000000;
 
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
		. += 4096;
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
	
	.stack : ALIGN(4k) {
		. += 4096;
		PROVIDE(stack_bottom = .);
		. += 4096 * 4;
		PROVIDE(stack_top = .);
		. += 4096;
	}
	
	PROVIDE(heap_start = .);
}
