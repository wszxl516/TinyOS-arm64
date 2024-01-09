#![allow(non_snake_case)]
#![allow(dead_code)]

pub mod TCR_EL1 {
    use crate::def_reg_fn;

    pub const BITS48_256TB: usize = 0b101 << 32;
    pub const TG1_16KB: usize = 0b01 << 30;
    pub const TG1_4KB: usize = 0b10 << 30;
    pub const TG1_64KB: usize = 0b11 << 30;
    pub const SH1_INNER_SHAREABLE: usize = 0b11 << 28;
    pub const ORGN1_NORMAL_OUTER: usize = 0b01 << 26;
    pub const IRGN1_NORMAL_INNER: usize = 0b01 << 24;
    pub const WALKS_ON_MISS: usize = 0b0 << 23;
    pub const NO_WALKS_ON_MISS: usize = 0b1 << 23;

    #[inline(always)]
    pub fn T1SZ(value: usize) -> usize {
        (value & 0x3F) << 16
    }

    pub const TG0_4KB: usize = 0b00;
    pub const TG0_64KB: usize = 0b01;
    pub const TG0_16KB: usize = 0b10;
    pub const SH0_INNER_SHAREABLE: usize = 0b11 << 12;
    pub const ORGN0_NORMAL_OUTER: usize = 0b01 << 10;
    pub const IRGN0_NORMAL_INNER: usize = 0b01 << 8;

    pub const TTBR0_WALKS_ON_MISS: usize = 0b0 << 7;
    pub const TTBR0_NO_WALKS_ON_MISS: usize = 0b1 << 7;

    #[inline(always)]
    pub fn T0SZ(value: usize) -> usize {
        value & 0x3F
    }

    def_reg_fn!(usize, TCR_EL1);
}
