//! ARM Generic Interrupt Controller v2.
#![allow(dead_code)]

use lazy_static::lazy_static;
use tock_registers::interfaces::{Readable, Writeable};

use crate::common::sync::Mutex;
use reg::{GICC, GICD};

use crate::config::{GICC_BASE, GICD_BASE};
use crate::mm::VirtAddr;

mod reg;

const NUM_IRQ: usize = 128;

pub type HandlerFn = fn(u32) -> i32;

pub struct GICv2 {
    gicd_base: usize,
    gicc_base: usize,
    handlers: [Option<HandlerFn>; NUM_IRQ],
}

lazy_static! {
    pub static ref GIC_V2: Mutex<GICv2> = {
        let gic = GICv2::form_addr(
            VirtAddr::from_phy(GICD_BASE).as_usize(),
            VirtAddr::from_phy(GICC_BASE).as_usize(),
        );
        Mutex::new(gic)
    };
}

unsafe impl Sync for GICv2 {}

impl GICv2 {
    const HANDLER_NONE: Option<HandlerFn> = None;

    pub const fn form_addr(gicd_base: usize, gicc_base: usize) -> GICv2 {
        Self {
            gicd_base,
            gicc_base,
            handlers: [Self::HANDLER_NONE; NUM_IRQ],
        }
    }
    pub fn set_handler(&mut self, irq: u32, handler: HandlerFn){
        self.handlers[irq as usize].replace( handler);
    }
    fn gicc(&self) -> &'static mut GICC {
        unsafe { &mut *(self.gicc_base as *mut GICC) }
    }
    fn gicd(&self) -> &'static mut GICD {
        unsafe { &mut *(self.gicd_base as *mut GICD) }
    }
    pub fn init(&self) {
        self.gicd().ctlr.set(0);
        self.gicc().ctlr.set(0);
        self.gicc().pmr.set(0xff);
        self.gicc().bpr.set(0);
        self.gicd().ctlr.set(1);
        self.gicc().ctlr.set(1);
    }
    pub fn enable(&self, interrupt: u32) {
        let gicd = self.gicd();
        gicd.isenabler[(interrupt / 32) as usize].set(1 << (interrupt % 32));
    }

    pub fn disable(&self, interrupt: u32) {
        let gicd = self.gicd();
        gicd.icenabler[(interrupt / 32) as usize].set(1 << (interrupt % 32));
    }

    fn set_priority(&self, interrupt: u32, priority: u32) {
        let gicd = self.gicd();

        let shift = (interrupt % 4) * 8;
        let mut value: u32 = gicd.ipriority[(interrupt / 4) as usize].get();
        value &= !(0xff << shift);
        value |= priority << shift;
        gicd.ipriority[(interrupt / 4) as usize].set(value);
    }

    fn set_config(&self, interrupt: u32, config: TriggerMode) {
        let config = match config {
            TriggerMode::Edge => 0,
            TriggerMode::Level => 1,
        };
        let gicd = self.gicd();
        let shift = (interrupt % 16) * 2;
        let mut value: u32 = gicd.icfgr[(interrupt / 16) as usize].get();
        value &= !(0x03 << shift);
        value |= config << shift;
        gicd.icfgr[(interrupt / 16) as usize].set(value);
    }
    pub fn setup_irq(&self, irq: u32, mode: TriggerMode) {
        self.set_config(irq, mode);
        self.set_priority(irq, 0);
        self.gicd_set_target(irq, 0);
        self.clear_pending(irq);
        self.enable(irq);
    }
    pub fn clear_pending(&self, irq: u32) {
        self.gicd().icpendr[(irq / 32) as usize].set(1 << (irq % 32));
    }
    pub fn probe_pending(&self, irq: u32) -> bool {
        let gicd = self.gicd();
        gicd.ispendr[(irq / 32) as usize].get() & (1 << (irq % 32)) != 0
    }
    pub fn gicd_set_target(&self, irq: u32, cpuid: u32) {
        let shift = (irq % 4) * 8;
        let mut value = self.gicd().itargetsr[(irq / 4) as usize].get();
        value &= !0xff << shift;
        value |= (cpuid + 1) << shift;
        self.gicd().itargetsr[(irq / 4) as usize].set(value)
    }
    pub fn fetch_irq(&self) -> Option<u32> {
        for i in 0..64 {
            match self.probe_pending(i) {
                true => return Some(i),
                false => {}
            }
        }
        None
    }
}

pub enum TriggerMode {
    Edge = 0,
    Level = 1,
}

pub fn setup_irq(irq: u32, mode: TriggerMode, handler: HandlerFn) {
    match GIC_V2.lock() {
        mut gic => {
            gic.setup_irq(irq, mode);
            gic.set_handler(irq, handler);

        }
    }
}

pub fn fetch_handler(irq: u32) -> Option<HandlerFn> {
    match GIC_V2.lock() {
        gic => {
            gic.handlers[irq as usize]

        }
    }
}

pub fn enable_irq(irq: u32) {
    match GIC_V2.lock() {
        gic => {
            gic.enable(irq)

        }
    }
}

pub fn fetch_irq() -> Option<u32> {
    match GIC_V2.lock() {
        gic => {
            gic.fetch_irq()

        }
    }
}

pub fn ack_irq(irq: u32) {
    match GIC_V2.lock() {
        gic => {
            gic.clear_pending(irq)
        }
    }
}
