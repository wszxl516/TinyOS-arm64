#[allow(unused_imports)]
pub use gicv2::{ack_irq, fetch_handler, fetch_irq, IntId, setup_irq, Trigger};

use crate::arch::timer::setup_timer;
use crate::mm::PhyAddr;
use crate::reg_write_p;

mod gicv2;

pub mod entry;
pub mod reg;
pub mod macros;
pub mod psci;
pub mod trap;
mod timer;

mod mmu;
mod panic;


pub static BOOT_ARGS: [PhyAddr; 4] = [PhyAddr::new(0); 4];

pub fn init() {
    extern "C" { pub fn exception_base(); }
    reg_write_p!(VBAR_EL1, exception_base as usize);
    setup_timer();
}
