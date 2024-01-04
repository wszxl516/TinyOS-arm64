use core::alloc::{GlobalAlloc, Layout};
use core::ptr::NonNull;

use crate::config::MEM_SIZE;
use linked_list_allocator::Heap;

use crate::common::sync::SpinNoIrqLock;
use crate::lds_address;
use crate::mm::{VirtAddr, PAGE_SIZE};


#[alloc_error_handler]
pub fn handle_alloc_error(layout: Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}

#[global_allocator]
pub static ALLOCATOR: LockedHeap = LockedHeap::empty();
pub struct LockedHeap(SpinNoIrqLock<Heap>);
unsafe impl GlobalAlloc for LockedHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0
            .lock()
            .allocate_first_fit(layout)
            .ok()
            .map_or(core::ptr::null_mut(), |allocation| allocation.as_ptr())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0
            .lock()
            .deallocate(NonNull::new_unchecked(ptr), layout)
    }
}
impl LockedHeap {
    pub const fn empty() -> Self {
        Self(SpinNoIrqLock::new(Heap::empty()))
    }
    pub fn init(&self, start: usize, end: usize) {
        unsafe { self.0.lock().init(start as *mut u8, end) };
    }
    #[allow(unused)]
    pub fn get(&self) -> &SpinNoIrqLock<Heap> {
        &self.0
    }
}


#[allow(dead_code)]
pub fn page_alloc(pages: usize) -> VirtAddr {
    let layout = Layout::from_size_align(pages * PAGE_SIZE, PAGE_SIZE).unwrap();
    let addr = unsafe { ALLOCATOR.alloc_zeroed(layout) };
    VirtAddr::new(addr.addr())
}

#[allow(dead_code)]
pub fn page_free(start: VirtAddr, pages: usize) {
    unsafe {
        ALLOCATOR.dealloc(
            start.as_mut_ptr(),
            Layout::from_size_align(pages * PAGE_SIZE, PAGE_SIZE).unwrap(),
        )
    }
}

pub fn init_heap() {
    let mem_size = MEM_SIZE - (lds_address!(heap_start) - lds_address!(text_start));
    let heap_start = lds_address!(heap_start);
    use crate::{pr_delimiter, pr_notice};
    pr_delimiter!();
    pr_notice!("{: ^56} \r\n", "Heap init");
    pr_delimiter!();
    pr_notice!(
        "start: {:#016x}  size: {:#010x}\n",
        lds_address!(heap_start),
        mem_size
    );
    pr_delimiter!();
    ALLOCATOR.init(heap_start, mem_size);
}
