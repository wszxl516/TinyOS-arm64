.section .init
.global _entry
_entry:
    /*all cache invalid*/
    mrs      x0, MPIDR_EL1
    and      x0, x0, #0xFF
    CBZ      x0, primary                       // If core 0, run the primary init
    b        others                         // Else, run secondary init
primary:
    ic      iallu
    tlbi    vmalle1is
    dsb     ish
    /* NEON setup */
    /* enable fpu */
    mov     x8, #(0x3 << 20)
    msr     cpacr_el1, x8
    /*disable exception interrupt*/
    msr     mdscr_el1, xzr
    isb
    adrp    x8, stack_top
    mov     sp, x8
    bl      setup_trap
    bl      parse_featrue
    ldr     x8, =kernel_main
    br      x8


others:
  wfi
  b        others