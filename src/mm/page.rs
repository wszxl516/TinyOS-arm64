use crate::{align_down, align_up};
use crate::mm::address::{PAGE_SIZE, PhyAddr, VirtAddr};
use crate::mm::attr::PTEFlags;
use crate::mm::entry::PTE;
use crate::mm::page_alloc;

pub const PAGE_ENTRY_COUNT: usize = 512;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PageTable {
    pub entrys: [PTE; PAGE_ENTRY_COUNT],
}

impl PageTable {
    pub const fn new() -> Self {
        Self {
            entrys: [PTE::empty(); PAGE_ENTRY_COUNT],
        }
    }
    pub fn alloc() -> &'static mut Self {
        let addr = page_alloc(1);
        unsafe { &mut *(VirtAddr::new(addr).as_mut_ptr() as *mut Self) }
    }
    pub fn map_page(&mut self, va: VirtAddr, pa: PhyAddr, flags: PTEFlags, force: bool) {
        let entry = self.find_entry(va);
        if unsafe { *entry }.is_valid() && force {
            unsafe { *entry = PTE::new_entry(pa, flags, false) }
        } else {
            unsafe { *entry = PTE::new_entry(pa, flags, false) }
        }
    }
    #[allow(dead_code)]
    pub fn unmap_page(&mut self, va: VirtAddr) {
        let va_start = VirtAddr::new(align_down!(va.as_usize(), PAGE_SIZE));
        let entry = self.find_entry(va_start);
        unsafe { *entry }.clear()
    }
    pub fn map_range(
        &mut self,
        va: VirtAddr,
        pa: PhyAddr,
        size: usize,
        flags: PTEFlags,
        force: bool,
    ) {
        let mut va_start = VirtAddr::new(align_down!(va.as_usize(), PAGE_SIZE));
        let mut pa_start = PhyAddr::new(align_down!(pa.as_usize(), PAGE_SIZE));
        let size = align_up!(size, PAGE_SIZE);
        loop {
            self.map_page(va_start, pa_start, flags, force);
            va_start = VirtAddr::new(va_start.as_usize() + PAGE_SIZE);
            pa_start = PhyAddr::new(pa_start.as_usize() + PAGE_SIZE);
            if va_start.as_usize() >= va.align_down().as_usize() + size {
                break;
            }
        }
    }
    #[allow(dead_code)]
    pub fn unmap_range(&mut self, va: VirtAddr, size: usize) {
        let mut va_start = VirtAddr::new(align_down!(va.as_usize(), PAGE_SIZE));
        let size = align_up!(size, PAGE_SIZE);
        loop {
            self.unmap_page(va_start);
            va_start = VirtAddr::new(va_start.as_usize() + PAGE_SIZE);
            if va_start.as_usize() >= va.align_down().as_usize() + size {
                break;
            }
        }
    }

    pub fn addr(&self) -> VirtAddr {
        VirtAddr::new(self as *const Self as usize)
    }
    pub fn find_entry(&mut self, va: VirtAddr) -> *mut PTE {
        let mut entry;
        let mut page_table = self;
        for level in [3, 2, 1] {
            entry = &mut (page_table.entrys[va.vpn(level)]);
            if entry.is_valid() {
                page_table = unsafe { &mut (*entry.as_table()) }
            } else {
                page_table = PageTable::alloc();
                let addr = page_table.addr().as_phy();
                *entry = PTE::new_page(addr)
            }
        }
        &mut page_table.entrys[va.vpn(0)]
    }
}
