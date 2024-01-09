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
    sub     sp, sp, 34 * 8
    push    x28, x29
    push    x26, x27
    push    x24, x25
    push    x22, x23
    push    x20, x21
    push    x18, x19
    push    x16, x17
    push    x14, x15
    push    x12, x13
    push    x10, x11
    push    x8, x9
    push    x6, x7
    push    x4, x5
    push    x2, x3
    push    x0, x1

    mrs     x0, sp_el0
    mrs     x1, elr_el1
    mrs     x2, spsr_el1
    stp     lr, x0, [sp, 30 * 8]
    stp     x1, x2, [sp, 32 * 8]
    mov     x0, sp
    bl      \handler
    b       .exit_exception
.endm

local_func .exit_exception
    ldp     x10, x11, [sp, 32 * 8]
    ldp     lr, x9, [sp, 30 * 8]
    msr     sp_el0, x9
    msr     elr_el1, x10
    msr     spsr_el1, x11
    pop     x0, x1
    pop     x2, x3
    pop     x4, x5
    pop     x6, x7
    pop     x8, x9
    pop     x10, x11
    pop     x12, x13
    pop     x14, x15
    pop     x16, x17
    pop     x18, x19
    pop     x20, x21
    pop     x22, x23
    pop     x24, x25
    pop     x26, x27
    pop     x28, x29

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


