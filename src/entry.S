.global _entry
_entry:
    /* NEON setup */
    /* enable fpu */
    mov x8, #(0x3 << 20)
    msr cpacr_el1, x8
    isb
    adrp    x8, boot_stack_top
    mov     sp, x8
    ldr     x8, =main
    br      x8    
