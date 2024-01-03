pub use address::{PhyAddr, VirtAddr};
pub use page::{PageTable};
pub use entry::PTE;
pub use attr::PTEFlags;


mod address;
pub mod heap;
mod page;
pub mod flush;
mod attr;
mod entry;
mod mem;

pub const PAGE_SIZE: usize = 0x1000;
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

#[macro_export]
macro_rules! addr2slice {
    ($addr: expr, $size: expr, $slice_type: ty) => {
        unsafe { core::slice::from_raw_parts_mut(($addr) as *mut u8 as *mut $slice_type, $size) }
    }
}
pub fn init(){
    heap::init_heap();
    mem::init_kernel_space();
}
