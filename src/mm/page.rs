#![allow(dead_code)]

use crate::{addr2slice, align_up};
use crate::mm::attr::PTEFlags;
use crate::mm::entry::PTE;
use crate::mm::heap::page_alloc;

use super::{PAGE_SIZE, PhyAddr, VirtAddr};

pub const USER_START: usize = 0x0000_0000_0000_0000;
pub const USER_END: usize = 0x0000_FFFF_FFFF_FFFF;
pub const KERNEL_START: usize = 0xFFFF_0000_0000_0000;
pub const KERNEL_END: usize = 0xFFFF_FFFF_FFFF_FFFF;
pub const PHYS_VIRT_OFFSET: usize = KERNEL_START;
pub const VA_MAX_BITS: usize = 48;
pub const PAGE_ENTRY_COUNT: usize = 512;

pub struct PageTable {
    root_addr: PhyAddr,
}

impl PageTable {
    pub const L0: usize = 0;
    pub const L1: usize = 1;
    pub const L2: usize = 2;
    pub const L3: usize = 3;
    pub const fn new() -> Self {
        Self {
            root_addr: PhyAddr::new(0),
        }
    }
    pub fn init(&mut self) {
        self.root_addr = page_alloc(1).as_phy()
    }
    fn alloc_page(&mut self) -> PhyAddr {
        page_alloc(1).as_phy()
    }
    pub const fn root_phy_addr(&self) -> PhyAddr {
        self.root_addr
    }
    #[inline]
    fn entrys<'a>(&mut self) -> &'a mut [PTE] {
        addr2slice!(
            self.root_phy_addr().into_vaddr().as_mut_ptr(),
            PAGE_ENTRY_COUNT,
            PTE
        )
    }
    fn find_entry(&mut self, vaddr: VirtAddr, level: usize) -> Option<&mut PTE> {
        assert!(level <= 3);
        let mut entrys = self.entrys();
        for level in &[3, 2, 1][0..3 - level] {
            entrys = entrys[vaddr.vpn(*level)].as_page(|| self.alloc_page())?
        }
        Some(&mut entrys[vaddr.vpn(level)])
    }
    pub fn query(&mut self, vaddr: VirtAddr, level: usize) -> Option<(PhyAddr, PTEFlags)> {
        let entry = self.find_entry(vaddr, level)?;
        if entry.is_unused() || (level > 0 && !entry.is_block()) {
            return None;
        }
        Some((
            PhyAddr::new(entry.as_phy_addr().as_usize() + vaddr.page_offset()),
            entry.flags(),
        ))
    }

    fn find_block(&mut self, vaddr: VirtAddr) -> Option<&mut PTE> {
        self.find_entry(vaddr, Self::L1)
    }
    //2M block
    pub fn map_block_2m(
        &mut self,
        vaddr: VirtAddr,
        phy_addr: PhyAddr,
        flags: PTEFlags,
        force: bool,
    ) {
        match self.find_block(vaddr.align_down_2m()) {
            None => panic!("can not find entry of addr: {:#x}", vaddr.as_usize()),
            Some(entry) => {
                if !entry.is_unused() && !force {
                    panic!(
                        "{} => {:#x} {:#x} is mapped before mapping",
                        vaddr,
                        phy_addr.as_usize(),
                        entry.0
                    );
                }
                *entry = PTE::new_entry(phy_addr.align_down(), flags, true);
            }
        }
    }
    pub fn unmap_block_2m(&mut self, vaddr: VirtAddr) {
        match self.find_block(vaddr.align_down_2m()) {
            None => panic!("can not find entry of addr: {:#x}", vaddr.as_usize()),
            Some(entry) => {
                entry.clear();
            }
        }
    }
    pub fn map_page(&mut self, vaddr: VirtAddr, phy_addr: PhyAddr, flags: PTEFlags, force: bool) {
        match self.find_entry(vaddr.align_down_4k(), Self::L0) {
            None => panic!("can not find entry of addr: {:#x}", vaddr.as_usize()),
            Some(entry) => {
                if !entry.is_unused() && !force {
                    panic!(
                        "{} => {:#x} {:#x} is mapped before mapping",
                        vaddr,
                        phy_addr.as_usize(),
                        entry.0
                    );
                }
                *entry = PTE::new_entry(phy_addr.align_down(), flags, false);
            }
        }
    }

    pub fn unmap_page(&mut self, vaddr: VirtAddr) {
        match self.find_entry(vaddr.align_down_4k(), Self::L0) {
            None => panic!("can not find entry of addr: {:#x}", vaddr.as_usize()),
            Some(entry) => {
                entry.clear();
            }
        }
    }

    pub fn map_area(
        &mut self,
        vaddr: VirtAddr,
        phy_addr: PhyAddr,
        size: usize,
        flags: PTEFlags,
        force: bool,
    ) {
        let mut va_start = vaddr.align_down_4k().as_usize();
        let mut pa_start = phy_addr.align_down().as_usize();
        let size = align_up!(size, PAGE_SIZE);
        let end = va_start + size;
        while va_start < end {
            let start_pa = PhyAddr::new(pa_start);
            self.map_page(VirtAddr::new(va_start), start_pa, flags, force);
            va_start += PAGE_SIZE;
            pa_start += PAGE_SIZE;
        }
    }

    pub fn unmap_area(&mut self, vaddr: VirtAddr, size: usize) {
        let mut va_start = vaddr.align_down_4k().as_usize();
        let size = align_up!(size, PAGE_SIZE);
        let end = va_start + size;
        while va_start < end {
            self.unmap_page(VirtAddr::new(va_start));
            va_start += PAGE_SIZE;
        }
    }
}
