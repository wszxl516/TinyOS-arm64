// offset  	event type 	            description
// 0x000 	Synchronous Exception 	EL is using SP_EL0 stack
// 0x080 	IRQ 	                EL is using SP_EL0 stack
// 0x100 	FIQ 	                EL is using SP_EL0 stack
// 0x180 	SError 	                EL is using SP_EL0 stack
// 0x200 	Synchronous Exception 	EL is using SP_ELx stack
// 0x280 	IRQ 	                EL is using SP_ELx stack
// 0x300 	FIQ 	                EL is using SP_ELx stack
// 0x380 	SError 	                EL is using SP_ELx stack
// 0x400 	Synchronous Exception 	From lower EL in AArch64
// 0x480 	IRQ 	                From lower EL in AArch64
// 0x500 	FIQ 	                From lower EL in AArch64
// 0x580 	SError 	                From lower EL in AArch64
// 0x600 	Synchronous Exception 	From lower EL in AArch32
// 0x680 	IRQ 	                From lower EL in AArch32
// 0x700 	FIQ                 	From lower EL in AArch32
// 0x780 	SError 	                From lower EL in AArch32

.equ CONTEXT_SIZE, 34 * 8

.macro context_with handler
    sub sp, sp, 34 * 8
    stp x0, x1, [sp]
    stp x2, x3, [sp, 2 * 8]
    stp x4, x5, [sp, 4 * 8]
    stp x6, x7, [sp, 6 * 8]
    stp x8, x9, [sp, 8 * 8]
    stp x10, x11, [sp, 10 * 8]
    stp x12, x13, [sp, 12 * 8]
    stp x14, x15, [sp, 14 * 8]
    stp x16, x17, [sp, 16 * 8]
    stp x18, x19, [sp, 18 * 8]
    stp x20, x21, [sp, 20 * 8]
    stp x22, x23, [sp, 22 * 8]
    stp x24, x25, [sp, 24 * 8]
    stp x26, x27, [sp, 26 * 8]
    stp x28, fp, [sp, 28 * 8]

    mrs     x9,  SP_EL0
    mrs     x10, ELR_EL1
    mrs     x11, SPSR_EL1
    stp     lr, x9, [sp, 30 * 8]
    stp     x10, x11, [sp, 32 * 8]
    mov     x0, sp

    bl \handler
    b .exit_exception
.endm

local_func .exit_exception
    ldp     x10, x11, [sp, 32 * 8]
    ldp     lr, x9, [sp, 30 * 8]
    msr     sp_el0, x9
    msr     elr_el1, x10
    msr     spsr_el1, x11

    ldp     x28, fp, [sp, 28 * 8]
    ldp     x26, x27, [sp, 26 * 8]
    ldp     x24, x25, [sp, 24 * 8]
    ldp     x22, x23, [sp, 22 * 8]
    ldp     x20, x21, [sp, 20 * 8]
    ldp     x18, x19, [sp, 18 * 8]
    ldp     x16, x17, [sp, 16 * 8]
    ldp     x14, x15, [sp, 14 * 8]
    ldp     x12, x13, [sp, 12 * 8]
    ldp     x10, x11, [sp, 10 * 8]
    ldp     x8, x9, [sp, 8 * 8]
    ldp     x6, x7, [sp, 6 * 8]
    ldp     x4, x5, [sp, 4 * 8]
    ldp     x2, x3, [sp, 2 * 8]
    ldp     x0, x1, [sp]

    add     sp, sp, #CONTEXT_SIZE
    eret


.section .text.vector
.p2align 11
.global exception_base
exception_base:

/* exceptions from current EL, using SP0 */
.org 0x0
local_func sync_exception_sp_el0
    context_with  invalid_exception

.org 0x080
local_func irq_sp_el0
    context_with  invalid_exception

.org 0x100
local_func fiq_sp_el0
    context_with  invalid_exception

.org 0x180
local_func error_exception_sp_el0
    context_with  invalid_exception

/* exceptions from current EL, using SPx */
.org 0x200
local_func sync_exception_sp_elx
    context_with  sync_exception
.org 0x280
local_func irq_sp_elx
    context_with  platform_irq

.org 0x300
local_func fiq_sp_elx
    context_with  platform_fiq

.org 0x380
local_func error_exception_sp_elx
    context_with  invalid_exception

/* exceptions from lower EL, running arm64 */
.org 0x400
local_func sync_exception_lower_el_64
    context_with  sync_exception

.org 0x480
local_func irq_lower_el_64
    context_with  platform_irq

.org 0x500
local_func fiq_lower_el_64
    context_with  platform_fiq

.org 0x580
local_func error_exception_lower_el_64
    context_with  invalid_exception


