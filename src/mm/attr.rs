use core::fmt::{Display, Formatter};

use bitflags::bitflags;

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct PTEFlags: usize {
        const R        = 1 << 0;
        const W        = 1 << 1;
        const X        = 1 << 2;
        const U        = 1 << 3;
        const D        = 1 << 4;
        const RW       = (1 << 0) | (1 << 1);
        const RX       = (1 << 0) | (1 << 2);
        const RWX      = (1 << 0) | (1 << 1) | (1 << 2);

    }
}

impl PTEFlags {
    const FLAG_STR: [char; 5] = ['r', 'w', 'x', 'u', 'd'];
}

impl Display for PTEFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        for i in 0usize..5 {
            if self.contains(Self::from_bits(1 << i).unwrap()) {
                write!(f, "{}", PTEFlags::FLAG_STR[i]).unwrap()
            } else {
                write!(f, "-").unwrap()
            }
        }
        Ok(())
    }
}

bitflags! {
    pub struct PTEAttr: usize {
        /// Whether the descriptor is valid.
        const VALID =       1 << 0;
        /// The descriptor gives the address of the next level of translation table or 4KB page.
        /// (not a 2M, 1G block)
        const NON_BLOCK =   1 << 1;
        /// Memory attributes index field.
        const ATTR_INDX =   0b111 << 2;
        /// Non-secure bit. For memory accesses from Secure state, specifies whether the output
        /// address is in Secure or Non-secure memory.
        const NS =          1 << 5;
        /// Access permission: accessible at EL0.
        const AP_EL0 =      1 << 6;
        /// Access permission: read-only.
        const AP_RO =       1 << 7;
        /// Shareability: Inner Shareable (otherwise Outer Shareable).
        const INNER =       1 << 8;
        /// Shareability: Inner or Outer Shareable (otherwise Non-shareable).
        const SHAREABLE =   1 << 9;
        /// The Access flag.
        const AF =          1 << 10;
        /// The not global bit.
        const NG =          1 << 11;
        /// Indicates that 16 adjacent translation table entries point to contiguous memory regions.
        const CONTIGUOUS =  1 <<  52;
        /// The Privileged execute-never field.
        const PXN =         1 <<  53;
        /// The Execute-never or Unprivileged execute-never field.
        const UXN =         1 <<  54;

        /// PXN limit for subsequent levels of lookup.
        const PXN_TABLE =           1 << 59;
        /// XN limit for subsequent levels of lookup.
        const XN_TABLE =            1 << 60;
        /// Access permissions limit for subsequent levels of lookup: access at EL0 not permitted.
        const AP_NO_EL0_TABLE =     1 << 61;
        /// Access permissions limit for subsequent levels of lookup: write access not permitted.
        const AP_NO_WRITE_TABLE =   1 << 62;
        /// For memory accesses from Secure state, specifies the Security state for subsequent
        /// levels of lookup.
        const NS_TABLE =            1 << 63;
    }
}

#[repr(usize)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum MemType {
    Device = 0,
    Normal = 1,
}

impl PTEAttr {
    const ATTR_INDEX_MASK: usize = 0b111_00;

    const fn from_mem_type(mem_type: MemType) -> Self {
        let mut bits = (mem_type as usize) << 2;
        if matches!(mem_type, MemType::Normal) {
            bits |= Self::INNER.bits() | Self::SHAREABLE.bits();
        }
        Self::from_bits_truncate(bits)
    }

    fn mem_type(&self) -> MemType {
        let idx = (self.bits() & Self::ATTR_INDEX_MASK) >> 2;
        match idx {
            0 => MemType::Device,
            1 => MemType::Normal,
            _ => panic!("Invalid memory attribute index"),
        }
    }
}

impl From<PTEAttr> for PTEFlags {
    fn from(attr: PTEAttr) -> Self {
        let mut flags = Self::empty();
        if attr.contains(PTEAttr::VALID) {
            flags |= Self::R;
        }
        if !attr.contains(PTEAttr::AP_RO) {
            flags |= Self::W;
        }
        if attr.contains(PTEAttr::AP_EL0) {
            flags |= Self::U;
            if !attr.contains(PTEAttr::UXN) {
                flags |= Self::X;
            }
        } else if !attr.intersects(PTEAttr::PXN) {
            flags |= Self::X;
        }
        if attr.mem_type() == MemType::Device {
            flags |= Self::D;
        }
        flags
    }
}

impl From<PTEFlags> for PTEAttr {
    fn from(flags: PTEFlags) -> Self {
        let mut attr = if flags.contains(PTEFlags::D) {
            Self::from_mem_type(MemType::Device)
        } else {
            Self::from_mem_type(MemType::Normal)
        };
        if flags.contains(PTEFlags::R) {
            attr |= Self::VALID;
        }
        if !flags.contains(PTEFlags::W) {
            attr |= Self::AP_RO;
        }
        if flags.contains(PTEFlags::U) {
            attr |= Self::AP_EL0 | Self::PXN;
            if !flags.contains(PTEFlags::X) {
                attr |= Self::UXN;
            }
        } else {
            attr |= Self::UXN;
            if !flags.contains(PTEFlags::X) {
                attr |= Self::PXN;
            }
        }
        attr
    }
}
