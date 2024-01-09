use lazy_static::lazy_static;

use crate::{reg_read_p, reg_update_p, reg_write_p};
use crate::arch::{IntId, setup_irq, Trigger};
use crate::common::sync::RwLock;
use crate::config::TIMER_IRQ;
use crate::task::scheduler;

lazy_static! {
    static ref TIMER: RwLock<Timer> = RwLock::new(Timer::new());
}

#[derive(Copy, Clone, Debug)]
pub struct Timer {
    clock_freq: u64,
    ms_ticks: u64,
}

impl Timer {
    pub const fn new() -> Self {
        Self {
            clock_freq: 0,
            ms_ticks: 0,
        }
    }
    pub fn init(&mut self) {
        self.clock_freq = reg_read_p!(CNTFRQ_EL0) as u64;
        self.ms_ticks = self.clock_freq / 1000;
        setup_irq(IntId::ppi(TIMER_IRQ), Trigger::Edge, timer_irq_handler);
        reg_update_p!(CNTP_CTL_EL0, 1);
    }
    #[allow(dead_code)]
    pub fn get_time_ms(&self) -> u64 {
        reg_read_p!(CNTPCT_EL0) as u64 * 1000 / self.clock_freq
    }
    #[allow(dead_code)]
    pub fn freq(&self) -> u64 {
        self.clock_freq
    }
}

fn timer_irq_handler(_irq: IntId) -> i32 {
    reg_write_p!(CNTP_TVAL_EL0, TIMER.read().ms_ticks * 100);
    scheduler::yield_current();
    0
}

pub fn setup_timer() {
    match TIMER.write() {
        mut lock => lock.init(),
    };
    timer_irq_handler(IntId::ppi(TIMER_IRQ));
}
