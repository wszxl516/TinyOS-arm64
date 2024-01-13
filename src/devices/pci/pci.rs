use core::fmt::{Debug, Display, Formatter};

use bitflags::bitflags;
use tock_registers::register_bitfields;
use tock_registers::registers::ReadWrite;

use crate::common::MMIO;
use crate::devices::macros::pci_addr;
use crate::devices::pci::bar::BarType;
use crate::mm::PhyAddr;

///https://wiki.osdev.org/PCI
use super::{
    bar::Bar,
    ids::{dev_type, find},
};

#[derive(Debug)]
#[repr(C)]
pub struct Head {
    pub vendor_id: u16,
    pub device_id: u16,
    command: *mut u16,
    pub status: *mut u16,
    revision_id: u8,
    prog_if: u8,
    pub sub_class: u8,
    pub class_code: u8,
    cache_line_size: u8,
    latency_timer: u8,
    pub header_type: u8,
    bist: u8,
}

impl Display for Head {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let (v, d) = find(self.vendor_id, self.device_id);
        let s = dev_type(self.class_code, self.sub_class);

        write!(
            f,
            "{} {:#04x}:{:#04x} [ {} {} {:#04x}:{:#04x} ]",
            s, self.class_code, self.sub_class, v, d, self.vendor_id, self.device_id
        )
    }
}

impl Head {
    pub fn read_header(base_addr: usize) -> Head {
        let base = MMIO::new(base_addr);
        Head {
            vendor_id: base.read(),
            device_id: base.offset(0x2).read(),
            command: (base_addr + 0x4) as *mut u16,
            status: (base_addr + 0x6) as *mut u16,
            revision_id: base.offset(0x8).read(),
            prog_if: base.offset(0x9).read(),
            sub_class: base.offset(0xa).read(),
            class_code: base.offset(0xb).read(),
            cache_line_size: base.offset(0xc).read(),
            latency_timer: base.offset(0xd).read(),
            header_type: base.offset(0xe).read(),
            bist: base.offset(0xf).read(),
        }
    }
}

impl Display for Header0 {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.header)
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct Header0 {
    pub header: Head,
    pub base_address_reg: [Bar; 6],
    card_bus_cis_pointer: usize,
    subsystem_vendor_id: u16,
    subsystem_id: u16,
    expansion_rom_base_address: *mut u32,
    pub capabilities_pointer: u8,
    pub interrupt_line: *mut u8,
    pub interrupt_pin: *mut u8,
    min_grant: u8,
    max_latency: u8,
    pub cap: Option<CapHeader>,
    pub base_addr: usize,
    pub bus: u8,
    pub device: u8,
    pub func: u8,
}
bitflags! {
    struct Command: u16{
        const PCI_COMMAND_IO_EN = 0x0001;
        const PCI_COMMAND_MEM_EN = 0x0002;
        const PCI_COMMAND_BUS_MASTER_EN = 0x0004;
        const PCI_COMMAND_SPECIAL_EN = 0x0008;
        const PCI_COMMAND_MEM_WR_INV_EN = 0x0010;
        const PCI_COMMAND_PAL_SNOOP_EN = 0x0020;
        const PCI_COMMAND_PERR_RESP_EN = 0x0040;
        const PCI_COMMAND_AD_STEP_EN = 0x0080;
        const PCI_COMMAND_SERR_EN = 0x0100;
        const PCI_COMMAND_FAST_B2B_EN = 0x0200;
    }
}
impl Header0 {
    pub fn new(base: usize, bus: u8, device: u8, func: u8) -> Option<Self> {
        let base_addr = pci_addr!(base, bus, device, func);
        let header = Head::read_header(base_addr);
        if header.vendor_id == u16::MAX && header.device_id == u16::MAX {
            return None;
        }
        let base = MMIO::new(base_addr);
        let cap = match base.offset(0x34).read::<u8>() {
            0 => None,
            _ => Some(CapHeader::from_addr(
                base_addr,
                base.offset(0x34).read::<u8>() as usize,
            )),
        };
        Some(Self {
            header,
            base_address_reg: [
                Bar::from_addr(base_addr + 0x10),
                Bar::from_addr(base_addr + 0x14),
                Bar::from_addr(base_addr + 0x18),
                Bar::from_addr(base_addr + 0x1c),
                Bar::from_addr(base_addr + 0x20),
                Bar::from_addr(base_addr + 0x24),
            ],
            card_bus_cis_pointer: base_addr + 0x28,
            subsystem_vendor_id: base.offset(0x2a).read(),
            subsystem_id: base.offset(0x2c).read(),
            expansion_rom_base_address: (base_addr + 0x30) as *mut u32,
            capabilities_pointer: base.offset(0x34).read(),
            interrupt_line: (base_addr + 0x3c) as *mut u8,
            interrupt_pin: (base_addr + 0x3d) as *mut u8,
            min_grant: base.offset(0x3e).read(),
            max_latency: base.offset(0x3f).read(),
            base_addr,
            bus,
            device,
            cap,
            func,
        })
    }
    pub fn enable(&self) {
        let mut cmd = Command::from_bits(unsafe { self.header.command.read_volatile() }).unwrap();
        cmd |= Command::PCI_COMMAND_IO_EN
            | Command::PCI_COMMAND_MEM_EN
            | Command::PCI_COMMAND_IO_EN
            | Command::PCI_COMMAND_MEM_EN
            | Command::PCI_COMMAND_BUS_MASTER_EN;
        unsafe { self.header.command.write_volatile(cmd.bits()) }
    }
    pub fn setup_bar(&mut self, id: usize, address: usize, cpu_address: PhyAddr) {
        // Disable IO and MEM decoding around BAR detection, as we fiddle with
        let cmd = Command::from_bits(unsafe { self.header.command.read_volatile() })
            .unwrap()
            .bits();
        unsafe {
            self.header.command.write_volatile(
                (Command::from_bits(cmd).unwrap()
                    & !(Command::PCI_COMMAND_IO_EN | Command::PCI_COMMAND_MEM_EN))
                    .bits(),
            )
        };
        match self.base_address_reg[id].bar_type {
            BarType::Memory => match self.base_address_reg[id].mem_bits {
                32 => self.base_address_reg[id].setup(address as u32, cpu_address.as_usize()),
                64 => {
                    self.base_address_reg[id].setup(address as u32, cpu_address.as_usize());
                    self.base_address_reg[id + 1]
                        .setup(address.overflowing_shr(32).0 as u32, cpu_address.as_usize());
                }
                _ => unreachable!(),
            },
            BarType::IO => match self.base_address_reg[id].mem_bits {
                32 => self.base_address_reg[id].setup(address as u32, cpu_address.as_usize()),
                64 => {
                    self.base_address_reg[id].setup(address as u32, cpu_address.as_usize());
                    self.base_address_reg[id + 1]
                        .setup(address.overflowing_shr(32).0 as u32, cpu_address.as_usize());
                }
                _ => unreachable!(),
            },
        }
        unsafe { self.header.command.write_volatile(cmd) };
    }
}

register_bitfields![u16,
    MC [
        Enable OFFSET(15) NUMBITS(1),
        Function_Mask  OFFSET(14) NUMBITS(1),
        Table_Size  OFFSET(0) NUMBITS(11),
    ]
];
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CapHeader {
    pub id: u8,
    pub next_pointer: u8,
    pub base_addr: usize,
}
impl CapHeader {
    pub fn from_addr(addr: usize, offset: usize) -> Self {
        let address = addr + offset;
        let base = MMIO::new(address);

        Self {
            id: base.read::<u8>(),
            next_pointer: base.offset(0x1).read(),
            base_addr: addr,
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CapMSIX {
    header: CapHeader,
    message_control: *mut ReadWrite<u16, MC::Register>,
    bar: u8,
    table_offset: u32,
    pending_bit_offset: u32,
}

impl CapMSIX {
    #[allow(dead_code)]
    pub fn from_addr(addr: usize, offset: usize) -> Self {
        let address = addr + offset;
        let base = MMIO::new(address);
        Self {
            header: CapHeader {
                id: base.read::<u8>(),
                next_pointer: base.offset(0x1).read(),
                base_addr: addr,
            },
            message_control: (address + 0x2) as *mut ReadWrite<u16, MC::Register>,
            bar: base.offset(0x4).read(),
            table_offset: base.offset(0x4).read::<u32>() & !0b11,
            pending_bit_offset: base.offset(0x8).read(),
        }
    }
}

#[allow(dead_code)]
#[repr(C)]
pub struct MsixTable {
    message_address_low: ReadWrite<u32>,
    message_address_high: ReadWrite<u32>,
    message_data: ReadWrite<u32>,
    vector_control: ReadWrite<u32>,
}
