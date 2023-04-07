#ifndef __EXCEPTION_H__
#define __EXCEPTION_H__
#include "common.h"
#include "stdtypes.h"

#define EC_SYSCALL (0b010101)

const char* exception_msg[] = {
    [0b000000] = "Unknown reason.",
    [0b000001] = "Trapped WF* instruction execution.",
    [0b000011] = "Trapped MCR or MRC access.",
    [0b000100] = "Trapped MCRR or MRRC access.",
    [0b000101] = "Trapped MCR or MRC access.",
    [0b000110] = "Trapped LDC or STC access.",
    [0b000111] = "Access to SME, SVE, Advanced SIMD or floating-point.",
    [0b001010] = "Trapped execution of an LD64B or ST64B* instruction.",
    [0b001100] = "Trapped MRRC access.",
    [0b001101] = "Branch Target Exception.",
    [0b001110] = "Illegal Execution state.",
    [0b010001] = "SVC instruction execution in AArch32 state.",
    [EC_SYSCALL] = "SVC instruction execution in AArch64 state.",
    [0b011000] =
        "Trapped MSR, MRS or System instruction execution in AArch64 state.",
    [0b011001] = "Access to SVE functionality trapped.",
    [0b011011] = "Exception from an access to a TSTART instruction.",
    [0b011100] =
        "Exception from a Pointer Authentication instruction authentication failure.",
    [0b011101] = "Access to SME functionality trapped.",
    [0b011110] = "Exception from a Granule Protection Check.",
    [0b100000] = "Instruction Abort from a lower Exception level.",
    [0b100001] = "Instruction Abort taken without a change in Exception level.",
    [0b100010] = "PC alignment fault exception.",
    [0b100100] = "Data Abort exception from a lower Exception level.",
    [0b100101] =
        "Data Abort exception taken without a change in Exception level.",
    [0b100110] = "SP alignment fault exception.",
    [0b100111] = "Memory Operation Exception.",
    [0b101000] = "Trapped floating-point exception taken from AArch32 state.",
    [0b101100] = "Trapped floating-point exception taken from AArch64 state.",
    [0b101111] = "SError interrupt..",
    [0b110000] = "Breakpoint exception from a lower Exception level.",
    [0b110001] =
        "Breakpoint exception taken without a change in Exception level.",
    [0b110010] = "Software Step exception from a lower Exception level.",
    [0b110011] =
        "Software Step exception taken without a change in Exception level.",
    [0b110100] = "Watchpoint exception from a lower Exception level.",
    [0b110101] =
        "Watchpoint exception taken without a change in Exception level.",
    [0b111000] = "BKPT instruction execution in AArch32 state.",
    [0b111100] = "BRK instruction execution in AArch64 state.",

};

typedef struct OPTIMIZATION_ALIGN(8) {
  /// General registers (x0..x30).
  usize regs[31];
  /// User Stack Pointer (SP_EL0).
  usize usp;
  /// Exception Link Register (ELR_EL1).
  usize elr;
  /// Saved Process Status Register (SPSR_EL1).
  usize spsr;

} trap_frame;

#endif  //__EXCEPTION_H__