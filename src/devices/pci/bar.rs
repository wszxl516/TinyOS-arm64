use core::fmt::{Debug, Formatter};

use tock_registers::fields::TryFromValue;
use tock_registers::interfaces::{Readable, Writeable};
use tock_registers::register_bitfields;
use tock_registers::registers::ReadWrite;

//https://wiki.osdev.org/PCI#Base_Address_Registers

#[repr(C)]
pub struct BarReg(ReadWrite<u32, STATUS::Register>);

register_bitfields![u32,
    STATUS [
        /// prefetchable
        PREFETCH OFFSET(3) NUMBITS(1),
        /// 64 bits address or 32
        ADDR_BITS OFFSET(1) NUMBITS(2),
        ///memory os io
        MEM_TYPE OFFSET(0) NUMBITS(1),
    ]
];
pub struct Bar {
    reg: &'static BarReg,
    pub bar_type: BarType,
    pub mem_bits: u8,
    pub prefetch: bool,
    pub mem_size: usize,
    pub addr: usize,
}

impl Debug for Bar {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Bar {{ {:#x} type: {:?}, bits: {}, prefetch: {}, size: {:#x} }}",
            self.reg as *const BarReg as usize,
            self.bar_type,
            self.mem_bits,
            self.prefetch,
            self.mem_size
        )
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum BarType {
    Memory = 0,
    IO = 1,
}

impl TryFromValue<u32> for BarType {
    type EnumType = BarType;

    fn try_from_value(v: u32) -> Option<Self::EnumType> {
        Some(match v {
            0 => BarType::Memory,
            1 => BarType::IO,
            _ => unreachable!(),
        })
    }
}

impl Bar {
    pub fn from_addr(addr: usize) -> Self {
        let reg = unsafe { &*(addr as *mut BarReg) };
        let old = reg.0.get();
        reg.0.set(u32::MAX);
        let bar_type = reg.0.read_as_enum(STATUS::MEM_TYPE).unwrap();
        let mem = match bar_type {
            BarType::Memory => reg.0.get() & 0xFFFFFFF0,
            BarType::IO => reg.0.get() & 0xFFFFFFFC,
        };
        let mem_size = match mem {
            0 => 0,
            _ => (!(mem << 4)) + 1,
        };

        let bar = Self {
            reg,
            bar_type,
            mem_bits: match reg.0.read(STATUS::ADDR_BITS) {
                0 => 32,
                2 => 64,
                _ => 0,
            },
            prefetch: match reg.0.read(STATUS::PREFETCH) {
                0 => false,
                1 => true,
                _ => unreachable!(),
            },
            mem_size: mem_size as usize,
            addr: 0,
        };
        reg.0.set(old);
        bar
    }
    pub fn setup(&mut self, address: u32, cpu_address: usize) {
        self.reg.0.set(address);
        self.addr = cpu_address;
    }
}
