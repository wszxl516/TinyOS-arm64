#![allow(unused_imports)]

use core::arch::asm;

pub use daif::DAIF;
pub use mair_el1::MAIR_EL1;
pub use sctrl_el1::SCTLR_EL1;
pub use tcr_el1::TCR_EL1;

use crate::{reg_read_p, reg_write_p};

mod daif;
mod mair_el1;
mod tcr_el1;
mod sctrl_el1;

#[allow(dead_code)]
#[inline(always)]
pub fn current_el() -> usize {
    reg_read_p!(CurrentEL) >> 2
}

#[allow(dead_code)]
#[inline(always)]
pub fn current_core() -> usize {
    const SMP_CPU_CLUSTER_SHIFT: usize = 8;
    const SMP_CPU_ID_BITS: usize = 24;
    let mpidr = reg_read_p!(mpidr_el1);
    return ((mpidr & ((1 << SMP_CPU_ID_BITS) - 1)) >> 8 << SMP_CPU_CLUSTER_SHIFT) | (mpidr & 0xff);
}

#[allow(dead_code)]
#[inline(always)]
pub fn thread_pointer() -> usize {
    reg_read_p!(TPIDR_EL1)
}

#[allow(dead_code)]
#[inline(always)]
pub fn set_thread_pointer(tp: usize) {
    reg_write_p!(TPIDR_EL1, tp)
}


#[allow(dead_code)]
#[inline(always)]
pub fn wfe() {
    unsafe {
        asm!("wfe", options(nomem, nostack));
    }
}

#[allow(dead_code)]
#[inline(always)]
pub fn wfi() {
    unsafe {
        asm!("wfi", options(nomem, nostack));
    }
}


#[allow(dead_code)]
#[inline(always)]
pub fn idle(cycles: usize) {
    for _ in 0..cycles {
        unsafe {
            asm!("nop", options(nomem, nostack));
        }
    }
}

#[allow(dead_code)]
#[inline(always)]
pub fn eret() -> ! {
    unsafe {
        asm!("eret", options(nomem, nostack));
        unreachable!()
    }
}

#[macro_export]
macro_rules! def_reg_fn {
    ($reg_type:ty, $reg_name:tt) => {
        #[inline]
        pub fn read() -> $reg_type {
            crate::reg_read_p!($reg_name)
        }
        #[inline]
        pub fn write(value: $reg_type) {
            crate::reg_write_p!($reg_name, value)
        }
        #[inline]
        pub fn set_field(value: $reg_type){
            let value = crate::reg_read_p!($reg_name) | value;
            crate::reg_write_p!($reg_name, value);
        }
        #[inline]
        pub fn get_field(value: $reg_type) -> $reg_type{
            crate::reg_read_p!($reg_name) & value
        }
        #[inline]
        pub fn is_contains(value: $reg_type) -> bool{
            (crate::reg_read_p!($reg_name) & value) == value
        }

    };
}
