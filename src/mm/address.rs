use core::fmt;
use core::fmt::{Formatter, LowerHex};

#[allow(dead_code)]
pub const USER_START: usize = 0x0000_0000_0000_0000;
#[allow(dead_code)]
pub const USER_END: usize = 0x0000_FFFF_FFFF_FFFF;
pub const KERNEL_START: usize = 0xFFFF_0000_0000_0000;
#[allow(dead_code)]
pub const KERNEL_END: usize = 0xFFFF_FFFF_FFFF_FFFF;
pub const PAGE_SIZE: usize = 0x1000;
pub const PHYS_VIRT_OFFSET: usize = KERNEL_START;
pub const VA_MAX_BITS: usize = 48;
pub const PAGE_ENTRY_COUNT: usize = 512;

#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhyAddr(usize);


#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtAddr(usize);

impl LowerHex for VirtAddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:#16x}", self.0)
    }
}


impl fmt::Debug for PhyAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("PA:{:#x}", self.0))
    }
}

impl fmt::Debug for VirtAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("VA:{:#x}", self.0))
    }
}
#[macro_export]
macro_rules! align_down {
    ($addr:expr, $page_size:ident) => {
        $addr & !($page_size - 1)
    };
}
#[macro_export]
macro_rules! align_up {
    ($addr:expr, $page_size:ident) => {
        ($addr + $page_size - 1) & !($page_size - 1)
    };
}

#[macro_export]
macro_rules! page_offset {
    ($addr:expr, $page_size:ident) => {
        $addr & ($page_size - 1)
    };
}
#[macro_export]
macro_rules! is_aligned {
    ($addr:expr, $page_size:ident) => {
        crate::page_offset!($addr, $page_size) == 0
    };
}

impl PhyAddr {
    pub const MAX: usize = (1 << 40) - 1;

    pub const fn new(pa: usize) -> Self {
        assert!(pa <= Self::MAX);
        Self(pa & Self::MAX)
    }
    pub const fn as_usize(&self) -> usize {
        self.0
    }
    pub const fn into_vaddr(self) -> VirtAddr {
        VirtAddr::from_phy(self.0)
    }
    pub const fn from_virt(vaddr: usize) -> Self {
        Self {
            0: vaddr & !PHYS_VIRT_OFFSET,
        }
    }
}

impl VirtAddr {
    pub const PAGE_DIR_OFFSET: usize = 12;
    pub const PAGE_L0_OFFSET: usize = 9 * 0 + Self::PAGE_DIR_OFFSET;
    pub const PAGE_L1_OFFSET: usize = 9 * 1 + Self::PAGE_DIR_OFFSET;
    pub const PAGE_L2_OFFSET: usize = 9 * 2 + Self::PAGE_DIR_OFFSET;
    pub const PAGE_L3_OFFSET: usize = 9 * 3 + Self::PAGE_DIR_OFFSET;
    pub const fn new(va: usize) -> Self {
        let top_bits = va >> VA_MAX_BITS;
        if top_bits != 0xffff {
            panic!("invalid KERNEL VA!")
        }
        Self(va)
    }
    pub const fn as_usize(&self) -> usize {
        self.0
    }
    #[allow(dead_code)]
    pub const fn as_ptr(&self) -> *const u8 {
        self.as_usize() as _
    }
    pub const fn as_mut_ptr(&self) -> *mut u8 {
        self.as_usize() as _
    }
    pub const fn align_down(&self) -> Self {
        Self(align_down!(self.0, PAGE_SIZE))
    }
    pub const fn from_phy(phy_addr: usize) -> Self {
        VirtAddr::new(phy_addr | PHYS_VIRT_OFFSET)
    }

    pub fn as_phy(&self) -> PhyAddr {
        PhyAddr::new(self.0 - PHYS_VIRT_OFFSET)
    }

    #[inline]
    pub const fn vpn(&self, level: usize) -> usize {
        assert!(level <= 3);
        match level {
            0 => (self.0 >> Self::PAGE_L0_OFFSET) & (PAGE_ENTRY_COUNT - 1),
            1 => (self.0 >> Self::PAGE_L1_OFFSET) & (PAGE_ENTRY_COUNT - 1),
            2 => (self.0 >> Self::PAGE_L2_OFFSET) & (PAGE_ENTRY_COUNT - 1),
            3 => (self.0 >> Self::PAGE_L3_OFFSET) & (PAGE_ENTRY_COUNT - 1),
            _ => unreachable!()
        }
    }
}
