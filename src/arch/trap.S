#include "macros.S"
.arch armv8.2-a+crc
.cpu cortex-a76

.macro SAVE_EL_REG
    mrs     x9, SP_EL0
    mrs     x10, ELR_EL1
    mrs     x11, SPSR_EL1
    stp     x30, x9, [sp, 30 * 8]
    stp     x10, x11, [sp, 32 * 8]
.endm

.macro RESTORE_EL_REG
    ldp     x10, x11, [sp, 32 * 8]
    ldp     x30, x9, [sp, 30 * 8]
    msr     sp_el0, x9
    msr     elr_el1, x10
    msr     spsr_el1, x11
.endm

.macro WITH_CONTEXT_CALL    func
    sub sp, sp, (34 * 8)
    SAVE_GENERIC_REG    0   30
    SAVE_EL_REG
    mov     x0, sp
    bl      \func
    RESTORE_EL_REG
    RESTORE_GENERIC_REG 0   30
    add sp, sp, 34 * 8
.endm

.align 12
.global trap_vector
trap_vector:
// ------------------------------------------------------------
// Current EL with SP0
// ------------------------------------------------------------
.align 12
sync_current_el_sp0:
    WITH_CONTEXT_CALL dump_error
    eret

.align 12
irq_current_el_sp0:
    WITH_CONTEXT_CALL handle_irq
    eret

.align 12
fiq_current_el_sp0:
    WITH_CONTEXT_CALL dump_error
    eret

.align 12
serror_current_el_sp0:
    WITH_CONTEXT_CALL dump_error
    eret

// ------------------------------------------------------------
// Current EL with SPx
// ------------------------------------------------------------

.align 12
sync_current_el_spx:
    WITH_CONTEXT_CALL dump_error
    eret

.align 12
irq_current_el_spx:
    WITH_CONTEXT_CALL handle_irq
    eret

.align 12
fiq_current_el_spx:
    WITH_CONTEXT_CALL dump_error
    eret

.align 12
serror_current_el_spx:
    WITH_CONTEXT_CALL dump_error
    eret

// ------------------------------------------------------------
// Lower EL using AArch64
// ------------------------------------------------------------

.align 12
sync_lower_el_aarch64:
    WITH_CONTEXT_CALL dump_error
    eret                  

.align 12
irq_lower_el_aarch64:
    WITH_CONTEXT_CALL handle_irq
    eret

.align 12
fiq_lower_el_aarch64:
    WITH_CONTEXT_CALL dump_error
    eret

.align 12
serror_lower_el_aarch64:
    WITH_CONTEXT_CALL dump_error
    eret
