use core::fmt;

use crate::{align_down, page_offset};
use crate::mm::page::{PAGE_ENTRY_COUNT, PHYS_VIRT_OFFSET, VA_MAX_BITS};

use super::{BLOCK_2M, PAGE_SIZE};

pub const PA_1TB_BITS: usize = 40;


#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhyAddr(usize);

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtAddr(usize);


impl fmt::LowerHex for PhyAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("{:#x}", self.0))
    }
}

impl fmt::LowerHex for VirtAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("{:#x}", self.0))
    }
}

impl fmt::Display for PhyAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("{:#x}", self.0))
    }
}

impl fmt::Display for VirtAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("{:#x}", self.0))
    }
}

impl PhyAddr {
    pub const MAX: usize = (1 << PA_1TB_BITS) - 1;

    pub const fn new(pa: usize) -> Self {
        Self(pa & Self::MAX)
    }
    pub const fn as_usize(&self) -> usize {
        self.0
    }
    pub const fn into_vaddr(self) -> VirtAddr {
        VirtAddr::from_phy(self.0)
    }
    pub const fn align_down(&self) -> Self {
        Self(align_down!(self.0, PAGE_SIZE))
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
    pub const  USER_STACK_START: usize = 0x80000000;
    pub const USER_START: usize = 0x00400000;
    pub const fn new(va: usize) -> Self {
        let top_bits = va >> VA_MAX_BITS;
        if top_bits != 0 && top_bits != 0xffff {
            panic!("invalid VA!")
        }
        Self(va)
    }
    pub const fn as_usize(&self) -> usize {
        self.0
    }
    pub const fn as_mut_ptr(&self) -> *mut u8 {
        self.as_usize() as _
    }
    pub const fn align_down_4k(&self) -> Self {
        Self(align_down!(self.0, PAGE_SIZE))
    }
    pub const fn align_down_2m(&self) -> Self {
        Self(align_down!(self.0, BLOCK_2M))
    }
    pub const fn page_offset(&self) -> usize {
        page_offset!(self.0, PAGE_SIZE)
    }
    pub const fn from_phy(phy_addr: usize) -> Self {
        VirtAddr::new(phy_addr | PHYS_VIRT_OFFSET)
    }
    pub const fn as_phy(&self) -> PhyAddr {
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
