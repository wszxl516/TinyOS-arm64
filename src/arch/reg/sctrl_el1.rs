//https://developer.arm.com/documentation/101548/0002/AArch64-registers/AArch64-register-descriptions/AArch64-other-register-description/SCTLR-EL1--System-Control-Register--EL1-
#![allow(non_snake_case)]
#![allow(dead_code)]

pub mod SCTLR_EL1 {
    use crate::def_reg_fn;

    //EL1 MPU(PMSAv8-64) or MMU(VMSAv8-64) enabled
    pub const M: usize = 1 << 0;
    //Alignment fault checking enabled when executing at EL1 or EL0.
    pub const A: usize = 1 << 1;
    //Stage 1 Cache ability control, for data accesses.
    pub const C: usize = 1 << 2;
    //SP Alignment(16-byte) check enable.
    pub const SA: usize = 1 << 3;
    //SP Alignment(16-byte) check enable for EL0
    pub const SA0: usize = 1 << 4;
    //Non-aligned access. This bit controls generation of Alignment faults at EL1 and EL0 under certain conditions.
    pub const NAA: usize = 0 << 6;
    // Any attempt at EL0 using AArch64 to execute an MRS, MSR(register), or MSR(immediate) instruction that accesses the AArch64-DAIF is trapped.
    pub const UMA: usize = 0 << 9;
    // Enable EL0 Access to the following instructions:
    //AArch64 CFP RCTX, DVP RCT and CPP RCTX instructions.
    pub const EN_RCTX: usize = 1 << 10;
    //Stage 1 instruction access Cache ability control, for accesses at EL0 and EL1
    pub const I: usize = 1 << 12;
    //Controls enabling of pointer authentication  of instruction addresses in the EL1&0 translation regime.
    pub const EN_DB: usize = 1 << 13;

    def_reg_fn!(usize, SCTLR_EL1);
}