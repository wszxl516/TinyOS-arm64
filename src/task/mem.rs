use crate::align_up;
use crate::mm::{PAGE_SIZE, PageTable, PhyAddr, PTEFlags, VirtAddr};
use crate::mm::heap::page_alloc;

#[repr(transparent)]
pub struct UserSpace{
    page: PageTable
}

impl UserSpace{
    pub const  USER_STACK_START: usize = 0x80000000;
    pub const USER_START: usize = 0x00400000;
    pub const USR_STACK_SIZE: usize = PAGE_SIZE * 4;

    pub fn empty()-> Self{
        Self{page: PageTable::empty()}
    }
    pub fn new()-> Self{
        let mut page = PageTable::empty();
        page.init();
        Self{page}
    }
    pub fn load_bin(&mut self, data: &[u8]) -> (usize, usize){
        let bin_size = align_up!(data.len(), PAGE_SIZE);
        let text_addr = page_alloc(bin_size / PAGE_SIZE);
        text_addr.copy_from(data);
        let text_start = VirtAddr::new(Self::USER_START);
        self.page.map_area(text_start, text_addr.as_phy(), bin_size, PTEFlags::RWX | PTEFlags::U, true);
        let stack_addr = page_alloc(Self::USR_STACK_SIZE / PAGE_SIZE);
        let stack_start = VirtAddr::new(Self::USER_STACK_START);
        self.page.map_area(stack_start, stack_addr.as_phy(), Self::USR_STACK_SIZE, PTEFlags::RW | PTEFlags::U, true);
        (text_start.as_usize(), stack_start.as_usize() + Self::USR_STACK_SIZE)
    }
    pub const fn root_addr(&self) -> PhyAddr{
        self.page.root_addr()
    }
}
