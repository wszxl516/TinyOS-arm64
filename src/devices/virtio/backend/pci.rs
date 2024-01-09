use core::fmt::{Display, Formatter};
use core::mem::size_of;

use tock_registers::interfaces::{Readable, Writeable};
use tock_registers::registers::{ReadOnly, ReadWrite};

use crate::{pr_notice, reg_read_a};
use crate::devices::pci::bus::PCIBus;
use crate::devices::virtio::blk::{DeviceStatus, Transport};
use crate::mm::VirtAddr;

//http://docs.oasis-open.org/virtio/virtio/v1.3/virtio-v1.3.html
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum CapType {
    /* Common configuration */
    VirtioPciCapCommonCfg = 1,
    /* Notifications */
    VirtioPciCapNotifyCfg = 2,
    /* ISR Status */
    VirtioPciCapIsrCfg = 3,
    /* Device specific configuration */
    VirtioPciCapDeviceCfg = 4,
    /* PCI configuration access */
    VirtioPciCapPciCfg = 5,
}

impl From<u8> for CapType {
    fn from(value: u8) -> Self {
        unsafe { core::mem::transmute_copy(&value) }
    }
}


#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub struct VirtioCap {
    cap_vndr: u8,
    /* Generic PCI field: PCI_CAP_ID_VNDR */
    cap_next: u8,
    /* Generic PCI field: next ptr. */
    cap_len: u8,
    /* Generic PCI field: capability length */
    cfg_type: CapType,
    /* Identifies the structure. */
    bar: u8,
    /* Where to find it. */
    padding: [u8; 3],
    /* Pad to full dword. */
    offset: u32,
    /* Offset within bar. */
    length: u32,
    /* Length of the structure, in bytes. */
    base_addr: usize,
    self_offset: usize,
}

#[repr(C)]
struct VirtioPciNotifyCap {
    cap: VirtioCap,
    notify_off_multiplier: u32,
    /* Multiplier for queue_notify_off. */
}

#[repr(C)]
struct VirtioPciCfgCap {
    cap: VirtioCap,
    pci_cfg_data: [u8; 4],
    /* Data for BAR access. */
}

#[repr(C)]
struct VirtioPciIsr {
    cap: VirtioCap,
    isr: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BlkConfig {
    pub capacity_low: u32,
    pub capacity_high: u32,
    pub size_max: u32,
    pub seg_max: u32,
    pub cylinders: u16,
    pub heads: u8,
    pub sectors: u8,
    pub blk_size: u32,
    pub physical_block_exp: u8,
    pub alignment_offset: u8,
    pub min_io_size: u16,
    pub opt_io_size: u32,
}

#[repr(C)]
pub struct VirtioPciCommonCfg {
    pub device_feature_select: ReadWrite<u32>,
    /* read-write */
    pub device_feature: ReadOnly<u32>,
    /* read-only for driver */
    pub driver_feature_select: ReadWrite<u32>,
    /* read-write */
    pub driver_feature: ReadWrite<u32>,
    /* read-write */
    pub msix_config: ReadWrite<u16>,
    /* read-write */
    pub num_queues: ReadOnly<u16>,
    /* read-only for driver */
    pub device_status: ReadWrite<u8>,
    /* read-write */
    pub config_generation: ReadOnly<u8>,
    /* read-only for driver */
    /* About a specific virtqueue. */
    pub queue_select: ReadWrite<u16>,
    /* read-write */
    pub queue_size: ReadWrite<u16>,
    /* read-write */
    pub queue_msix_vector: ReadWrite<u16>,
    /* read-write */
    pub queue_enable: ReadWrite<u16>,
    /* read-write */
    pub queue_notify_off: ReadOnly<u16>,
    /* read-only for driver */
    pub queue_desc: ReadWrite<u64>,
    /* read-write */
    pub queue_driver: ReadWrite<u64>,
    /* read-write */
    pub queue_device: ReadWrite<u64>,
    /* read-write */
}

impl VirtioCap {
    pub fn from_addr(addr: usize, offset: usize) -> Self {
        let address = addr + offset;
        Self {
            cap_vndr: reg_read_a!(address, u8),
            cap_next: reg_read_a!(address + 1, u8),
            cap_len: reg_read_a!(address + 2, u8),
            cfg_type: CapType::from(reg_read_a!(address + 3, u8)),
            bar: reg_read_a!(address + 4, u8),
            padding: [
                reg_read_a!(address + 5, u8),
                reg_read_a!(address + 6, u8),
                reg_read_a!(address + 7, u8),
            ],
            offset: reg_read_a!(address + 8, u32).to_le(),
            length: reg_read_a!(address + 12, u32).to_le(),
            base_addr: addr,
            self_offset: offset,
        }
    }
    pub fn next(&self) -> Option<Self> {
        match self.cap_next {
            0 => None,
            _ => Some(Self::from_addr(self.base_addr, self.cap_next as usize)),
        }
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct VirtioBlkTrans {
    pub config: Option<BlkConfig>,
    common_cfg: Option<*mut VirtioPciCommonCfg>,
    isr_cfg: Option<*mut u8>,
    notify_reg: Option<*mut u16>,
    notify_cfg: Option<*mut VirtioPciNotifyCap>,
    irq: u32,
}


impl Transport for VirtioBlkTrans {
    fn capacity(&self) -> u64 {
        match self.config() {
            None => 0,
            Some(config) => (config.capacity_high as u64) << 32 | config.capacity_low as u64
        }
    }

    fn blk_size(&self) -> u64 {
        match self.config() {
            None => 0,
            Some(config) => config.blk_size as u64
        }
    }

    fn status(&self) -> DeviceStatus {
        self.status()
    }

    fn reset(&mut self) {
        self.reset()
    }

    fn read_device_features(&mut self) -> u64 {
        self.read_device_features()
    }

    fn write_driver_features(&mut self, driver_features: u64) {
        self.write_driver_features(driver_features)
    }

    fn max_queue_size(&self) -> u32 {
        self.max_queue_size()
    }

    fn notify(&mut self, queue: u32) {
        unsafe { self.isr_reg().write_volatile(1); }
        self.notify(queue)
    }

    fn set_status(&mut self, status: DeviceStatus) {
        self.set_status(status)
    }

    fn queue_set(&mut self, queue: u32, size: u32, desc: usize, driver: usize, device: usize) {
        self.queue_set(queue, size, desc, driver, device)
    }

    fn queue_used(&mut self, queue: u32) -> bool {
        self.queue_used(queue)
    }

    fn init(&mut self) {
        self.init()
    }

    fn finish_init(&mut self) {
        self.finish_init()
    }
}

impl Display for VirtioBlkTrans {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.config)
    }
}

unsafe impl Sync for VirtioBlkTrans {}

unsafe impl Send for VirtioBlkTrans {}

#[allow(dead_code)]
impl VirtioBlkTrans {
    const BLK_VENDOR: u16 = 0x1af4;
    const BLK_DEVICE: u16 = 0x1001;

    pub fn from_pci(pci_bus: &mut PCIBus) -> Option<Self> {
        let mut common_cfg = None;
        let mut notify_cfg = None;
        let mut isr_reg = None;
        let mut notify_reg = None;
        let mut device_cfg = None;
        let irq = 1;
        match pci_bus.find_device(Self::BLK_VENDOR, Self::BLK_DEVICE) {
            None => return None,
            Some(mut pci) => {
                pr_notice!("PCI: {:02}.{:02}.{:02} {}\n", pci.bus, pci.device, pci.func, pci);
                pci.enable();
                unsafe { pci.header.status.write_volatile(1 << 3) }
                let mut cap;
                let c = pci.cap.unwrap();
                // pr_info!("{:#x?}", c);
                cap = VirtioCap::from_addr(c.base_addr, c.next_pointer as usize);
                loop {
                    match cap.next() {
                        None => break,
                        Some(vc) => {
                            cap = vc;
                            let base_addr = match pci.base_address_reg[cap.bar as usize].addr {
                                0 => {
                                    let address = pci_bus.mem[2]
                                        .alloc(pci.base_address_reg[cap.bar as usize].mem_size)
                                        .unwrap();
                                    pci.setup_bar(cap.bar as usize, address.0, address.1);
                                    address.1.as_usize()
                                }
                                _ => pci.base_address_reg[cap.bar as usize].addr
                            };
                            match cap.cfg_type {
                                CapType::VirtioPciCapCommonCfg => {
                                    common_cfg = Some(VirtAddr::from_phy(base_addr + cap.offset as usize).as_mut_ptr() as *mut VirtioPciCommonCfg);
                                }
                                CapType::VirtioPciCapNotifyCfg => {
                                    notify_reg = Some(VirtAddr::from_phy(base_addr + cap.offset as usize).as_mut_ptr() as *mut u16);
                                    notify_cfg = Some(VirtAddr::from_phy(cap.base_addr + cap.self_offset).as_mut_ptr() as *mut VirtioPciNotifyCap)
                                }
                                CapType::VirtioPciCapIsrCfg => {
                                    isr_reg = Some(VirtAddr::from_phy(base_addr + cap.offset as usize).as_mut_ptr())
                                }
                                CapType::VirtioPciCapDeviceCfg => {
                                    device_cfg = Some(unsafe { *(VirtAddr::from_phy(base_addr + cap.offset as usize).as_mut_ptr() as *mut BlkConfig) })
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        Some(Self { config: device_cfg, common_cfg, isr_cfg: isr_reg, notify_reg, notify_cfg, irq })
    }
    fn common_reg(&self) -> &'static mut VirtioPciCommonCfg {
        match self.common_cfg {
            None => panic!("VirtioPciCommonCfg is none!"),
            Some(reg) => unsafe { &mut *reg }
        }
    }
    fn notify_reg(&self) -> *mut u16 {
        match self.notify_reg {
            None => panic!("notify_reg is none!"),
            Some(reg) => reg
        }
    }
    fn isr_reg(&self) -> *mut u8 {
        match self.isr_cfg {
            None => panic!("notify_reg is none!"),
            Some(reg) => reg
        }
    }

    fn ack_interrupt(&mut self) {
        match self.isr_cfg {
            None => {}
            Some(isr) => {
                unsafe { isr.read_volatile() };
            }
        }
    }
    pub fn status(&self) -> DeviceStatus {
        DeviceStatus::from_bits_truncate(self.common_reg().device_status.get())
    }
    #[inline]
    pub fn reset(&mut self) {
        let virtio = self.common_reg();
        virtio.device_status.set(DeviceStatus::empty().bits());
        virtio.device_status.set(
            (DeviceStatus::from_bits_truncate(virtio.device_status.get()) | DeviceStatus::ACKNOWLEDGE)
                .bits(),
        );
        virtio.device_status.set(
            (DeviceStatus::from_bits_truncate(virtio.device_status.get()) | DeviceStatus::DRIVER).bits(),
        );
    }
    pub const fn config(&self) -> Option<BlkConfig> {
        self.config
    }

    fn read_device_features(&mut self) -> u64 {
        let reg = self.common_reg();
        reg.device_feature_select.set(0);
        let mut device_features_bits = reg.device_feature.get().into();
        reg.device_feature_select.set(1);
        device_features_bits += (reg.device_feature.get() as u64) << 32;
        device_features_bits
    }

    fn write_driver_features(&mut self, driver_features: u64) {
        let reg = self.common_reg();
        reg.device_feature_select.set(0);
        reg.driver_feature.set(driver_features as u32);
        reg.device_feature_select.set(1);
        reg.driver_feature.set((driver_features >> 32) as u32);
    }

    pub(crate) fn max_queue_size(&self) -> u32 {
        self.common_reg().queue_size.get() as u32
    }

    pub(crate) fn notify(&mut self, queue: u32) {
        self.common_reg().queue_select.set(queue as u16);
        let queue_notify_off = self.common_reg().queue_notify_off.get();
        let notify = self.notify_cfg.unwrap();
        let offset_bytes = usize::from(queue_notify_off) * (unsafe { &mut *notify }).notify_off_multiplier as usize;
        let index = offset_bytes / size_of::<u16>();
        unsafe { self.notify_reg().add(index).write(queue as u16); }
    }

    fn set_status(&mut self, status: DeviceStatus) {
        self.common_reg().device_status.set(status.bits());
    }

    pub(crate) fn queue_set(
        &mut self,
        queue: u32,
        size: u32,
        desc: usize,
        driver: usize,
        device: usize,
    ) {
        self.common_reg().queue_select.set(queue as u16);
        self.common_reg().queue_size.set(size as u16);
        self.common_reg().queue_desc.set(desc as u64);
        self.common_reg().queue_driver.set(driver as u64);
        self.common_reg().queue_device.set(device as u64);
        self.common_reg().queue_enable.set(1);
    }

    pub(crate) fn queue_used(&mut self, queue: u32) -> bool {
        self.common_reg().queue_select.set(queue as u16);
        self.common_reg().queue_enable.get() != 0
    }


    pub fn init(&mut self) {
        self.set_status(DeviceStatus::ACKNOWLEDGE);
        self.set_status(DeviceStatus::DRIVER);

        let features = self.read_device_features();
        self.write_driver_features(features);
        self.set_status(DeviceStatus::FEATURES_OK);
    }
    pub fn finish_init(&mut self) {
        self.set_status(DeviceStatus::DRIVER_OK);
    }
}
