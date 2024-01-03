use crate::mm::attr::PTEFlags;
use crate::mm::entry::PTE;
use crate::mm::heap::page_alloc;
use crate::{addr2slice, align_up};

#[allow(dead_code)]
pub const USER_START: usize = 0x0000_0000_0000_0000;
#[allow(dead_code)]
pub const USER_END: usize = 0x0000_FFFF_FFFF_FFFF;
pub const KERNEL_START: usize = 0xFFFF_0000_0000_0000;
#[allow(dead_code)]
pub const KERNEL_END: usize = 0xFFFF_FFFF_FFFF_FFFF;
pub const PHYS_VIRT_OFFSET: usize = KERNEL_START;
pub const VA_MAX_BITS: usize = 48;
pub const PAGE_ENTRY_COUNT: usize = 512;

use super::{PhyAddr, VirtAddr, PAGE_SIZE};

pub struct PageTable {
    base_addr: PhyAddr,
}
impl PageTable {
    pub const fn new() -> Self{
        Self{base_addr: PhyAddr::new(0)}
    }
    pub fn init(&mut self) {
        self.base_addr = VirtAddr::new(page_alloc(1)).as_phy()
    }
    fn alloc_page(&mut self) -> PhyAddr {
        VirtAddr::new(page_alloc(1)).as_phy()
    }
    pub const fn root_phy_addr(&self) -> PhyAddr {
        self.base_addr
    }
    #[inline]
    fn entrys<'a>(&mut self) -> &'a mut [PTE] {
        addr2slice!(
            self.root_phy_addr().into_vaddr().as_mut_ptr(),
            PAGE_ENTRY_COUNT,
            PTE
        )
    }
    fn find_entry(&mut self, vaddr: VirtAddr) -> Option<&mut PTE> {
        let vpn = vaddr.vpn();
        Some(
            &mut self.entrys()[vpn.3].get_page(|| self.alloc_page())?[vpn.2]
                .get_page(|| self.alloc_page())?[vpn.1]
                .get_page(|| self.alloc_page())?[vpn.0],
        )
    }
    #[allow(dead_code)]
    pub fn query(&mut self, vaddr: VirtAddr) -> Option<(PhyAddr, PTEFlags)> {
        let entry = self.find_entry(vaddr)?;
        if entry.is_unused() {
            return None;
        }
        let off = vaddr.page_offset();
        Some((
            PhyAddr::new(entry.phy_addr().as_usize() + off),
            entry.flags(),
        ))
    }
    pub fn map(&mut self, vaddr: VirtAddr, phy_addr: PhyAddr, flags: PTEFlags, force: bool) {
        let entry = self.find_entry(vaddr).unwrap();
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

    #[allow(dead_code)]
    pub fn unmap(&mut self, vaddr: VirtAddr) {
        let entry = self.find_entry(vaddr).unwrap();
        if entry.is_unused() {
            panic!("{} is invalid before unmapping", vaddr);
        }
        entry.clear();
    }

    pub fn map_area(
        &mut self,
        vaddr: VirtAddr,
        phy_addr: PhyAddr,
        size: usize,
        flags: PTEFlags,
        force: bool,
    ) {
        let mut va_start = vaddr.align_down().as_usize();
        let mut pa_start = phy_addr.align_down().as_usize();
        let size = align_up!(size, PAGE_SIZE);
        let end = va_start + size;
        while va_start < end {
            let start_pa = PhyAddr::new(pa_start);
            self.map(VirtAddr::new(va_start), start_pa, flags, force);
            va_start += PAGE_SIZE;
            pa_start += PAGE_SIZE;
        }
    }

    #[allow(dead_code)]
    pub fn unmap_area(&mut self, vaddr: VirtAddr, size: usize) {
        let mut va_start = vaddr.align_down().as_usize();
        let size = align_up!(size, PAGE_SIZE);
        let end = va_start + size;
        while va_start < end {
            self.unmap(VirtAddr::new(va_start));
            va_start += PAGE_SIZE;
        }
    }
}
