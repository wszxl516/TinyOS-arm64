use crate::addr2slice;
use crate::mm::{PAGE_SIZE, PhyAddr};
use crate::mm::attr::{PTEAttr, PTEFlags};
use crate::mm::page::PAGE_ENTRY_COUNT;

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
        let mut attr = PTEAttr::from(flags) | PTEAttr::AF;
        if !is_block {
            attr |= PTEAttr::NON_BLOCK;
        }
        Self(attr.bits() | (phy_addr.as_usize() & Self::PHYS_ADDR_MASK))
    }

    pub fn new_table(phy_addr: PhyAddr) -> Self {
        let attr = PTEAttr::NON_BLOCK | PTEAttr::VALID;
        Self(attr.bits() | (phy_addr.as_usize() & Self::PHYS_ADDR_MASK))
    }

    pub fn phy_addr(&self) -> PhyAddr {
        PhyAddr::new(self.0 & Self::PHYS_ADDR_MASK)
    }
    pub fn flags(&self) -> PTEFlags {
        PTEAttr::from_bits_truncate(self.0).into()
    }
    pub fn is_unused(&self) -> bool {
        self.0 == 0
    }
    fn is_present(&self) -> bool {
        PTEAttr::from_bits_truncate(self.0).contains(PTEAttr::VALID)
    }
    fn is_block(&self) -> bool {
        !PTEAttr::from_bits_truncate(self.0).contains(PTEAttr::NON_BLOCK)
    }
    pub fn clear(&mut self) {
        self.0 = 0
    }
    pub fn as_page<'a>(&mut self) -> Option<&'a mut [PTE]> {
        if !self.is_present() {
            None
        } else {
            assert!(!self.is_block());
            Some(addr2slice!(self.phy_addr().into_vaddr().as_mut_ptr(),PAGE_ENTRY_COUNT,PTE ))
        }
    }

    pub fn get_page<'a>(
        &mut self,
        mut allocator: impl FnMut() -> PhyAddr,
    ) -> Option<&'a mut [PTE]> {
        if self.is_unused() {
            let phy_addr = allocator();
            *self = PTE::new_table(phy_addr);
            Some(addr2slice!(phy_addr.into_vaddr().as_mut_ptr(), PAGE_ENTRY_COUNT, PTE))
        } else {
            self.as_page()
        }
    }
}
