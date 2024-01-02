.altmacro
.set fp, x29
.set lr, x30

.macro func_def _name, _align=2
    .global \_name
    .cfi_sections .debug_frame
    .section .text.asm, "ax"
    .type \_name, %function
    .cfi_startproc
    .align \_align
    \_name:
.endm

.macro func_end _name
    .cfi_endproc
    .size \_name, . - \_name
.endm

.macro local_func _name
    .type \_name,STT_FUNC; \_name:
.endm

.macro def_data name
    .global \name
    .type \name,STT_OBJECT; \name:
.endm

.macro end_data name
    .size \name, . - \name
.endm


.macro push ra, rb
    stp \ra, \rb, [sp,#-16]!
.endm

.macro pop ra, rb
    ldp \ra, \rb, [sp], #16
.endm
.macro putc c
    mov x8, #0x9000000
    mov x9, \c
    str x9, [x8, #0]
.endm

