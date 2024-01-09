//! ARM Generic Interrupt Controller v2.
#![allow(dead_code)]

use lazy_static::lazy_static;
use tock_registers::interfaces::{Readable, Writeable};

use reg::{GICC, GICD};
pub use types::{IntId, SgiData, Trigger};

use crate::common::sync::Mutex;
use crate::config::{GICC_BASE, GICD_BASE};
use crate::mm::VirtAddr;

mod reg;
mod types;

const NUM_IRQ: usize = 128;

pub type HandlerFn = fn(IntId) -> i32;

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
        gic.init();
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
    pub fn set_handler(&mut self, irq: IntId, handler: HandlerFn) {
        self.handlers[irq.0 as usize].replace(handler);
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
    pub fn enable(&self, interrupt: IntId) {
        let interrupt = interrupt.0;
        let gicd = self.gicd();
        gicd.isenabler[(interrupt / 32) as usize].set(1 << (interrupt % 32));
    }

    pub fn disable(&self, interrupt: IntId) {
        let interrupt = interrupt.0;

        let gicd = self.gicd();
        gicd.icenabler[(interrupt / 32) as usize].set(1 << (interrupt % 32));
    }

    fn set_priority(&self, interrupt: IntId, priority: u32) {
        let interrupt = interrupt.0;
        let gicd = self.gicd();

        let shift = (interrupt % 4) * 8;
        let mut value: u32 = gicd.ipriority[(interrupt / 4) as usize].get();
        value &= !(0xff << shift);
        value |= priority << shift;
        gicd.ipriority[(interrupt / 4) as usize].set(value);
    }

    fn set_config(&self, interrupt: IntId, config: Trigger) {
        let interrupt = interrupt.0;

        let config = match config {
            Trigger::Edge => 0,
            Trigger::Level => 1,
            Trigger::None => return
        };
        let gicd = self.gicd();
        let shift = (interrupt % 16) * 2;
        let mut value: u32 = gicd.icfgr[(interrupt / 16) as usize].get();
        value &= !(0x03 << shift);
        value |= config << shift;
        gicd.icfgr[(interrupt / 16) as usize].set(value);
    }
    pub fn setup_irq(&self, irq: IntId, mode: Trigger) {
        self.set_config(irq, mode);
        self.set_priority(irq, 0);
        self.gicd_set_target(irq, 0);
        self.clear_pending(irq);
        self.enable(irq);
    }
    pub fn clear_pending(&self, irq: IntId) {
        if irq.is_sgi() {
            self.gicd().sgiclear.set(self.gicd().sgiset.get());
        } else {
            let irq = irq.0;
            self.gicd().icpendr[(irq / 32) as usize].set(1 << (irq % 32));
        }
    }
    pub fn probe_pending(&self, irq: IntId) -> bool {
        let irq = irq.0;

        let gicd = self.gicd();
        gicd.ispendr[(irq / 32) as usize].get() & (1 << (irq % 32)) != 0
    }
    pub fn gicd_set_target(&self, irq: IntId, cpuid: u32) {
        let irq = irq.0;
        let shift = (irq % 4) * 8;
        let mut value = self.gicd().itargetsr[(irq / 4) as usize].get();
        value &= !0xff << shift;
        value |= (cpuid + 1) << shift;
        self.gicd().itargetsr[(irq / 4) as usize].set(value)
    }
    pub fn fetch_irq(&self) -> Option<IntId> {
        let sgi = self.gicd().sgiset.get();
        if sgi != 0 {
            return Some(IntId::sgi(((sgi >> 24) & 0xff) as u32));
        }
        for i in 0..64 {
            match self.probe_pending(IntId { 0: i }) {
                true => return Some(IntId { 0: i }),
                false => {}
            }
        }
        None
    }
    ///             let sgi = IntId::sgi(8);
    ///             gic.setup_irq(sgi, Trigger::None);
    ///             gic.set_handler(sgi, |_|{pr_info!("sgi");0});
    ///             gic.send_sgi(sgi, SgiData::List { affinity3: 0, affinity2: 0, affinity1: 0, target_list: 1 });
    pub fn send_sgi(&self, intid: IntId, target: SgiData) {
        assert!(intid.is_sgi());

        let sgi_value = match target {
            SgiData::All => {
                let irm = 0b1;
                (u64::from(intid.0 & 0x0f) << 24) | (irm << 40)
            }
            SgiData::List {
                affinity3,
                affinity2,
                affinity1,
                target_list,
            } => {
                let irm = 0b0;
                u64::from(target_list)
                    | (u64::from(affinity1) << 16)
                    | (u64::from(intid.0 & 0x0f) << 24)
                    | (u64::from(affinity2) << 32)
                    | (irm << 40)
                    | (u64::from(affinity3) << 48)
            }
        };
        self.gicd().sgiset.set(sgi_value);
    }
}


pub fn setup_irq(irq: IntId, mode: Trigger, handler: HandlerFn) {
    match GIC_V2.lock() {
        mut gic => {
            gic.setup_irq(irq, mode);
            gic.set_handler(irq, handler);
        }
    }
}

pub fn fetch_handler(irq: IntId) -> Option<HandlerFn> {
    match GIC_V2.lock() {
        gic => gic.handlers[irq.0 as usize],
    }
}

pub fn enable_irq(irq: IntId, is_enable: bool) {
    match GIC_V2.lock() {
        gic => {
            if is_enable {
                gic.enable(irq)
            } else {
                gic.disable(irq)
            }
        }
    }
}

pub fn fetch_irq() -> Option<IntId> {
    match GIC_V2.lock() {
        gic => gic.fetch_irq(),
    }
}

pub fn ack_irq(irq: IntId) {
    match GIC_V2.lock() {
        gic => gic.clear_pending(irq),
    }
}
