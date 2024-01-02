//! ARM Generic Timer.
use crate::{reg_read_p, reg_update_p, reg_write_p};
use crate::arch::{setup_irq, TriggerMode};
use crate::config::TIMER_IRQ;
use crate::task::scheduler;

static mut CLOCK_FREQ: u64 = 0;
static mut TRIGGER_TICKS: u64 = 0;


pub static mut TIMER: Timer = Timer::new();

pub struct Timer;

impl Timer {
    pub const fn new() -> Self {
        Self {}
    }
    pub fn init(&mut self) {
        let freq = reg_read_p!(CNTFRQ_EL0) as u64;
        unsafe {
            CLOCK_FREQ = freq;
            TRIGGER_TICKS = freq / 100;
        }
        timer_irq_handler(0);
        setup_irq(TIMER_IRQ, TriggerMode::Edge, timer_irq_handler);
        reg_update_p!(CNTP_CTL_EL0, 1);
    }
    #[allow(dead_code)]
    pub fn get_time_ms(&self) -> u64 {
        let freq = unsafe { CLOCK_FREQ };
        reg_read_p!(CNTPCT_EL0) as u64 * 1000 / freq
    }
    #[allow(dead_code)]
    pub fn freq(&self) -> u64 {
        unsafe { CLOCK_FREQ }
    }
}

pub fn timer_irq_handler(_irq: u32) -> i32 {
    let ticks = unsafe { TRIGGER_TICKS };
    reg_write_p!(CNTP_TVAL_EL0, ticks);
    scheduler::yield_current();
    0
}




