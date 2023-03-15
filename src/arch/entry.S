.section .init
.global _entry
_entry:
    /* NEON setup */
    /* enable fpu */
    mov     x30, (0x3 << 20)
    msr     cpacr_el1, x30
    isb
    ldr     x30, =stack_top
    mov     sp, x30
    ldr     x30, =kernel_main
    br      x30
