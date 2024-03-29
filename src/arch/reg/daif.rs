use core::arch::asm;

use crate::reg_read_p;

/* DAIF, Interrupt Mask Bits */
#[allow(dead_code)]
pub enum DAIF {
    /* Debug mask bit */
    Dbg,
    /* Asynchronous abort mask bit */
    Abt,
    /* IRQ mask bit */
    Irq,
    /* FIQ mask bit */
    Fiq,
    /*all of them*/
    All,
}
macro_rules! daif_op {
    ($reg_name: ident, $value: expr) => {
        unsafe { asm!(concat!("msr ", stringify!($reg_name),", #{bits}"), bits = const $value, options(nomem, nostack)) }
    };
}
impl DAIF {
    const DBG_BITS: usize = 1 << 3;
    const ABT_BITS: usize = 1 << 2;
    const IRQ_BITS: usize = 1 << 1;
    const FIQ_BITS: usize = 1 << 0;
    const ALL_BITS: usize = (1 << 3) | (1 << 2) | (1 << 1) | (1 << 0);
    #[inline(never)]
    #[allow(dead_code)]
    pub fn enable(&self) {
        match self {
            DAIF::Dbg => daif_op!(DAIFClr, Self::DBG_BITS),
            DAIF::Abt => daif_op!(DAIFClr, Self::ABT_BITS),
            DAIF::Irq => daif_op!(DAIFClr, Self::IRQ_BITS),
            DAIF::Fiq => daif_op!(DAIFClr, Self::FIQ_BITS),
            DAIF::All => daif_op!(DAIFClr, Self::ALL_BITS),
        }
    }
    #[inline(never)]
    #[allow(dead_code)]
    pub fn disable(&self) {
        match self {
            DAIF::Dbg => daif_op!(DAIFSet, Self::DBG_BITS),
            DAIF::Abt => daif_op!(DAIFSet, Self::ABT_BITS),
            DAIF::Irq => daif_op!(DAIFSet, Self::IRQ_BITS),
            DAIF::Fiq => daif_op!(DAIFSet, Self::FIQ_BITS),
            DAIF::All => daif_op!(DAIFSet, Self::ALL_BITS),
        }
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        match self {
            DAIF::Dbg => (reg_read_p!(DAIF) >> 9).eq(&1),
            DAIF::Abt => (reg_read_p!(DAIF) >> 8).eq(&1),
            DAIF::Irq => (reg_read_p!(DAIF) >> 7).eq(&1),
            DAIF::Fiq => (reg_read_p!(DAIF) >> 6).eq(&1),
            DAIF::All => (reg_read_p!(DAIF) & 0x3c0).eq(&0x3c0)
        }
    }
}
