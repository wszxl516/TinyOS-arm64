.altmacro
.macro SAVE_XN n
    str x\n, [sp,(\n )*8]
.endm

.macro LOAD_XN n
    ldr x\n , [sp,(\n )*8]
.endm

.macro SAVE_GENERIC_REG start end
    .set n, start
    .rept end - start
            SAVE_XN %n
            .set n, n + 1
    .endr
.endm

.macro RESTORE_GENERIC_REG start end
    .set n, start
    .rept end - start
            LOAD_XN %n
            .set n, n + 1
    .endr
.endm

    
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