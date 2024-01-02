use core::alloc::{GlobalAlloc, Layout};

use linked_list_allocator::LockedHeap;

use crate::{lds_address, pr_address, pr_delimiter, pr_notice, reg_write_p};
use crate::config::MEM_SIZE;
use crate::mm::address::{PhyAddr, VirtAddr};
use crate::mm::attr::PTEFlags;
use crate::mm::flush::{dsb_all, isb_all, tlb_all};
use crate::mm::page::PageTable;

pub const PAGE_SIZE: usize = 0x1000;
#[macro_export]
macro_rules! mem_set {
    ($address: expr, $len: expr, $value: expr) => {
        unsafe { core::slice::from_raw_parts_mut($address, $len).fill($value) }
    };
}

pub mod address;
pub mod attr;
pub mod entry;
pub mod flush;
pub mod page;

#[global_allocator]
pub static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[allow(dead_code)]
pub fn page_alloc(pages: usize) -> usize {
    let layout = Layout::from_size_align(pages * PAGE_SIZE, PAGE_SIZE).unwrap();
    let addr = unsafe { ALLOCATOR.alloc(layout) };
    mem_set!(addr, pages * PAGE_SIZE, 0);
    addr.addr()
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
    let heap_start = lds_address!(heap_start) as *mut u8;
    pr_delimiter!();
    pr_notice!("{: ^56} \r\n", "Heap init");
    pr_delimiter!();
    pr_notice!("start: {:#016x}  size: {:#010x}\n",lds_address!(heap_start), mem_size);
    pr_delimiter!();

    unsafe {
        ALLOCATOR.lock().init(heap_start, mem_size);
    }
}

#[link_section = ".data.kernel_root"]
static mut KERNEL_SPACE: PageTable = PageTable::new();

pub fn map_area(va_start: VirtAddr, pa_start: PhyAddr, size: usize, flags: PTEFlags, name: &str) {
    pr_delimiter!();
    pr_address!(
        name,
        va_start,
        size,
        flags
    );
    unsafe { &mut KERNEL_SPACE }.map_range(va_start, pa_start, size, flags, true);
}

#[no_mangle]
pub fn init_kernel_space() {
    pr_notice!("| {:<10} |  {:<18} | {:<10} | {:<5} |\n", "name", "addr", "size", "flags");
    //uart
    let (start, size) = (super::config::UART_ADDRESS, PAGE_SIZE);
    map_area(VirtAddr::from_phy(start),
             PhyAddr::new(start), size,
             PTEFlags::RW | PTEFlags::DEVICE, "uart");
    //gic
    let (start, size) = (super::config::GICD_BASE, super::config::GIC_SIZE * 2);
    map_area(VirtAddr::from_phy(start),
             PhyAddr::new(start), size,
             PTEFlags::RW | PTEFlags::DEVICE, "gic");
    //dtb
    let (start, size) = (super::arch::BOOT_ARGS[0], PAGE_SIZE * 4);
    map_area(start.into_vaddr(),
             start, size,
             PTEFlags::R, "dtb");
    //text
    let (start, size) = (lds_address!(text_start), lds_address!(text_end) - lds_address!(text_start));
    map_area(VirtAddr::new(start),
             PhyAddr::from_virt(start), size,
             PTEFlags::RX, "text");

    //rodata
    let (start, size) = (lds_address!(ro_start), lds_address!(ro_end) - lds_address!(ro_start));
    map_area(VirtAddr::new(start),
             PhyAddr::from_virt(start), size,
             PTEFlags::R, "rodata");

    //data
    let (start, size) = (lds_address!(data_start), lds_address!(data_end) - lds_address!(data_start));
    map_area(VirtAddr::new(start),
             PhyAddr::from_virt(start), size,
             PTEFlags::RW, "data");
    //bss
    let (start, size) = (lds_address!(bss_start), lds_address!(bss_end) - lds_address!(bss_start));
    map_area(VirtAddr::new(start),
             PhyAddr::from_virt(start), size,
             PTEFlags::RW, "bss");

    //stack
    let (start, size) = (lds_address!(stack_start), lds_address!(stack_end) - lds_address!(stack_start));
    map_area(VirtAddr::new(start),
             PhyAddr::from_virt(start), size,
             PTEFlags::RW, "stack");
    //symbols
    let (start, size) = (lds_address!(symbols_start), lds_address!(symbols_end) - lds_address!(symbols_start));
    map_area(VirtAddr::new(start),
             PhyAddr::from_virt(start), size,
             PTEFlags::R, "symbols");
    //heap
    let (start, size) = (lds_address!(heap_start), MEM_SIZE - (lds_address!(heap_start) - lds_address!(text_start)));
    map_area(VirtAddr::new(start),
             PhyAddr::from_virt(start), size,
             PTEFlags::RW, "heap");
    pr_delimiter!();
    let kernel_pg_addr = unsafe { &mut KERNEL_SPACE }.addr().as_usize();
    enable_table(kernel_pg_addr, true);
    enable_table(kernel_pg_addr, false);
}

pub fn enable_table(page_table_root: usize, is_kernel: bool) {
    if is_kernel {
        reg_write_p!(TTBR1_EL1, page_table_root)
    } else {
        reg_write_p!(TTBR0_EL1, page_table_root)
    }
    isb_all();
    dsb_all();
    tlb_all();
}
