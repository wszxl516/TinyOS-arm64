use core::arch::asm;

#[allow(named_asm_labels)]
#[no_mangle]
#[naked]
#[link_section = ".text.el1_entry"]
unsafe extern "C" fn el1_entry() -> ! {
    asm!(
    r#"
    /* NEON setup */
    /* enable fpu */
    mov x8, #(0x3 << 20)
    msr cpacr_el1, x8
    /*disable exception interrupt*/
    msr     daif, xzr
    msr     mdscr_el1, xzr
    isb
    mrs	x8, MPIDR_EL1
    and x8, x8, #0xff
    cmp	x8, xzr
    b.ne	other_core
    isb
    adrp x8, __stack_end
    mov	sp, x8
    bl  {init_mmu}
    //enable Stack pointer
    mov x8, #1
    msr SPSEL, x8
    // Jump to Rust code.
    ldr x8, ={main}
    br	x8

other_core:
    wfe
    b	other_core
	"#, main = sym crate::kernel_main,
        init_mmu = sym super::mmu::init_mmu,
    options(noreturn)
    )
}

#[allow(named_asm_labels)]
#[no_mangle]
#[naked]
#[link_section = ".text._entry"]
pub unsafe extern "C" fn _entry() {
    asm!(r#"
    adr    x8, {args}
    stp    x0, x1, [x8], #16
    stp    x2, x3, [x8]
    mrs    x8, CurrentEL
    and    x8, x8, #0b1100
    mov    x9, #0b100
    cmp    x8, x9
    b.eq   already_el1
    ldr    x9, =(3 << 28) | (3 << 22) | (1 << 20) | (1 << 11) | (0 << 25) |  (0 << 12) | (0 << 2) | (0 << 0)
    msr    sctlr_el1, x9
    ldr    x9, =1 << 31
    msr    hcr_el2, x9
    ldr    x9, =(3 << 4) | (1 << 10) | ( 1 << 0 )
    msr    scr_el3, x9
    ldr    x9, =(7 << 6) | (5 << 0)
    msr    spsr_el3, x9
    adr    x8, el1_entry
    msr    elr_el3, x8
    eret
already_el1:
    b      el1_entry
    "#,
        args = sym super::BOOT_ARGS,
        options(noreturn)
    )
}

