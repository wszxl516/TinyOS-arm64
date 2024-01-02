#![allow(dead_code)]

use crate::mm::address::PhyAddr;
use crate::mm::attr::{DescriptorAttr, PTEFlags};
use crate::mm::page::PageTable;
use crate::mm::PAGE_SIZE;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PTE(pub usize);

//Page table entry
impl PTE {
    const PHYS_ADDR_MASK: usize = PhyAddr::MAX & !(PAGE_SIZE - 1);

    pub const fn empty() -> Self {
        Self(0)
    }

    pub fn new_entry(phy_addr: PhyAddr, flags: PTEFlags, is_block: bool) -> Self {
        let mut attr = DescriptorAttr::from(flags) | DescriptorAttr::AF;
        if !is_block {
            attr |= DescriptorAttr::NON_BLOCK;
        }
        Self(attr.bits() | (phy_addr.as_usize() & Self::PHYS_ADDR_MASK))
    }

    pub fn new_page(phy_addr: PhyAddr) -> Self {
        let attr = DescriptorAttr::NON_BLOCK | DescriptorAttr::VALID;
        Self(attr.bits() | (phy_addr.as_usize() & Self::PHYS_ADDR_MASK))
    }
    pub fn as_table(&mut self) -> *mut PageTable {
        self.addr().into_vaddr().as_usize() as *mut PageTable
    }
    pub fn addr(&self) -> PhyAddr {
        PhyAddr::new(self.0 & Self::PHYS_ADDR_MASK)
    }
    pub fn flags(&self) -> PTEFlags {
        DescriptorAttr::from_bits_truncate(self.0).into()
    }
    pub fn is_valid(&self) -> bool {
        DescriptorAttr::from_bits_truncate(self.0).contains(DescriptorAttr::VALID)
    }
    pub fn is_block(&self) -> bool {
        !DescriptorAttr::from_bits_truncate(self.0).contains(DescriptorAttr::NON_BLOCK)
    }
    pub fn clear(&mut self) {
        self.0 = 0
    }
}
