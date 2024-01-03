#[allow(unused_imports)]
pub use gicv2::{setup_irq, TriggerMode};
use gicv2::GIC_V2;
pub use timer::TIMER;
use crate::mm::PhyAddr;

use crate::{reg_write_p};
pub mod entry;
pub mod reg;
pub mod macros;
pub mod psci;
pub mod trap;
mod timer;
mod gicv2;
mod mmu;
mod panic;


pub static BOOT_ARGS: [PhyAddr; 4] = [PhyAddr::new(0); 4];

pub fn init(){
    extern "C" { pub fn exception_base(); }
    reg_write_p!(VBAR_EL1, exception_base as usize);
    unsafe {
        TIMER.init();
        match GIC_V2.lock() {
            gic => {
                gic.init()
            }
        }
    }
}
