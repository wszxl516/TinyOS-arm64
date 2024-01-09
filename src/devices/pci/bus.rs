use core::fmt::{Debug, Formatter};
use arrayvec::{ArrayString, ArrayVec};
use fdt::node::FdtNode;
use crate::devices::macros::fdt_get;
use crate::devices::pci::pci::Header0;
use crate::mm::PhyAddr;

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum PciMemType {
    Config = 0b00,
    IOSpace = 0b01,
    MemorySpace32 = 0b10,
    MemorySpace64 = 0b11,
}

impl From<u32> for PciMemType {
    fn from(value: u32) -> Self {
        match value >> 24 & 0b11 {
            0b00 => PciMemType::Config,
            0b01 => PciMemType::IOSpace,
            0b10 => PciMemType::MemorySpace32,
            0b11 => PciMemType::MemorySpace64,
            _ => unreachable!(),
        }
    }
}

impl From<PciMemType> for u8 {
    fn from(value: PciMemType) -> Self {
        match value {
            PciMemType::Config => 0b00,
            PciMemType::IOSpace => 0b01,
            PciMemType::MemorySpace32 => 0b10,
            PciMemType::MemorySpace64 => 0b11,
        }
    }
}

pub type PciAddr = usize;

#[derive(Copy, Clone)]
pub struct PciMem {
    pub pci_addr: PciAddr,
    pub phy_addr: PhyAddr,
    pub size: usize,
    pub pci_type: PciMemType,
    used_size: usize,
}

impl Debug for PciMem {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "PciMem: {{ {:#x}, {:#x}, {:#x} {:?} }}",
            self.pci_addr, self.phy_addr, self.size, self.pci_type
        ))
    }
}

impl PciMem {
    pub fn alloc(&mut self, size: usize) -> Option<(PciAddr, PhyAddr)> {
        let ret = (
            PciAddr::from(self.pci_addr + self.used_size),
            PhyAddr::new(self.pci_addr + self.used_size),
        );
        self.used_size += size;
        if self.used_size >= self.size {
            return None;
        }
        Some(ret)
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct InterruptMap {
    device: u32,
    specifier: u32,
    parent: u32,
    parent_address: u32,
    parent_specifier: [u32; 3],
}

#[derive(Clone)]
pub struct PCIBus {
    pub reg: usize,
    pub reg_size: usize,
    pub mem: [PciMem; 3],
    pub name: ArrayString<64>,
    pub compatible: ArrayString<64>,
    pub device_type: ArrayString<64>,
    pub bus_range: [u32; 2],
    pub interrupt_map: ArrayVec<InterruptMap, 16>,
}

impl Debug for PCIBus {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("PCIBus {{name: {}, compatible: {},  type: {}, reg: {:#x}, size: {:#x},  mem: {:?},  bus: [{:#x} {:#x}] }}",
                                 self.name, self.compatible, self.device_type, self.reg, self.reg_size, self.mem, self.bus_range[0], self.bus_range[1]
        ))
    }
}

impl PCIBus {
    pub fn new() -> Self {
        Self {
            reg: 0,
            reg_size: 0,
            mem: [PciMem {
                pci_addr: 0,
                phy_addr: PhyAddr::new(0),
                size: 0,
                pci_type: PciMemType::Config,
                used_size: 0,
            }; 3],
            name: ArrayString::new_const(),
            compatible: ArrayString::new_const(),
            device_type: ArrayString::new_const(),
            bus_range: [0; 2],
            interrupt_map: ArrayVec::new(),
        }
    }
    pub fn from_fdt(node: &FdtNode) -> PCIBus {
        let mut pci = Self::new();
        pci.name.push_str(node.name);
        pci.compatible
            .push_str(node.property("compatible").unwrap().as_str().unwrap());
        pci.device_type
            .push_str(node.property("device_type").unwrap().as_str().unwrap());
        let mut data_slice = node.property("reg").unwrap().value;
        pci.reg = fdt_get!(data_slice, usize);

        pci.reg_size = fdt_get!(data_slice, usize);
        data_slice = node.property("ranges").unwrap().value;
        for i in 0..3 {
            let pci_type = fdt_get!(data_slice, u32);
            let pci_addr = fdt_get!(data_slice, usize);
            let phy_addr = fdt_get!(data_slice, usize);
            let size = fdt_get!(data_slice, usize);

            pci.mem[i] = PciMem {
                pci_type: PciMemType::from(pci_type),
                pci_addr,
                phy_addr: PhyAddr::new(phy_addr),
                size,
                used_size: 0,
            };
        }
        data_slice = node.property("bus-range").unwrap().value;
        pci.bus_range[0] = fdt_get!(data_slice,u32);
        pci.bus_range[1] = fdt_get!(data_slice,u32);
        pci.fetch_interrupt(&node);
        pci
    }
    pub fn fetch_interrupt(&mut self, node: &FdtNode) {
        match node.property("interrupt-map") {
            None => {}
            Some(interrupt_map) => {
                let mut map = InterruptMap {
                    device: 0,
                    specifier: 0,
                    parent: 0,
                    parent_address: 0,
                    parent_specifier: [0, 0, 0],
                };
                let mut interrupt_value = interrupt_map.value;
                while !interrupt_value.is_empty() {
                    let value = fdt_get!(interrupt_value, u64);
                    map.device = (value >> (32 + 11)) as u32;
                    let value = fdt_get!(interrupt_value, u64);
                    map.specifier = value as u32;

                    let value = fdt_get!(interrupt_value, u32);
                    map.parent = value;
                    let value = fdt_get!(interrupt_value, u64);
                    map.parent_address = value as u32;
                    map.parent_specifier = [
                        fdt_get!(interrupt_value, u32),
                        fdt_get!(interrupt_value, u32),
                        fdt_get!(interrupt_value, u32),
                    ];
                    self.interrupt_map.push(map);
                }
            }
        }
    }

    pub fn find_device(&self, vendor_id: u16, device_id: u16) -> Option<Header0> {
        for device in 0u8..255 {
            match Header0::new(self.reg, 0, device, 0) {
                None => return None,
                Some(pci) => {
                    if pci.header.vendor_id == vendor_id && pci.header.device_id == device_id {
                        return Some(pci);
                    }
                }
            }
        }
        return None;
    }
}