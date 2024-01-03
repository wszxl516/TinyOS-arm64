use crate::config::{GICD_BASE, GIC_SIZE, MEM_SIZE, UART_ADDRESS};
use crate::mm::{PTEFlags, PageTable, PhyAddr, VirtAddr, PAGE_SIZE};
use crate::{lds_address, pr_address, pr_delimiter, pr_notice, reg_write_p};
use lazy_static::lazy_static;
use crate::mm::flush::{dsb_all, isb_all, tlb_all};
use super::super::common::sync::{Mutex};
lazy_static! {
    #[link_section = ".data.kernel_root"]
    static ref KERNEL_SPACE: Mutex<PageTable> = {
        let mut k = PageTable::new();
        k.init();
        Mutex::new(k)
    };
}
pub fn map_area(va_start: VirtAddr, pa_start: PhyAddr, size: usize, flags: PTEFlags, name: &str) {
    pr_delimiter!();
    pr_address!(name, va_start, size, flags);
    match KERNEL_SPACE.lock() {
        mut lock => {
            lock.map_area(va_start, pa_start, size, flags, true)
        }
    }
}
pub fn init_kernel_space() {
    pr_notice!("{: ^56} \r\n", "Init Kernel page table");
    //uart
    let (start, size) = (UART_ADDRESS, PAGE_SIZE);
    map_area(
        VirtAddr::from_phy(start),
        PhyAddr::new(start),
        size,
        PTEFlags::RW | PTEFlags::D,
        "uart",
    );
    //gic
    let (start, size) = (GICD_BASE, GIC_SIZE * 2);
    map_area(
        VirtAddr::from_phy(start),
        PhyAddr::new(start),
        size,
        PTEFlags::RW | PTEFlags::D,
        "gic",
    );
    //text
    let (start, size) = (
        lds_address!(text_start),
        lds_address!(text_end) - lds_address!(text_start),
    );
    map_area(
        VirtAddr::new(start),
        PhyAddr::from_virt(start),
        size,
        PTEFlags::RX,
        "text",
    );

    //rodata
    let (start, size) = (
        lds_address!(ro_start),
        lds_address!(ro_end) - lds_address!(ro_start),
    );
    map_area(
        VirtAddr::new(start),
        PhyAddr::from_virt(start),
        size,
        PTEFlags::R,
        "rodata",
    );

    //data
    let (start, size) = (
        lds_address!(data_start),
        lds_address!(data_end) - lds_address!(data_start),
    );
    map_area(
        VirtAddr::new(start),
        PhyAddr::from_virt(start),
        size,
        PTEFlags::RW,
        "data",
    );
    //bss
    let (start, size) = (
        lds_address!(bss_start),
        lds_address!(bss_end) - lds_address!(bss_start),
    );
    map_area(
        VirtAddr::new(start),
        PhyAddr::from_virt(start),
        size,
        PTEFlags::RW,
        "bss",
    );

    //stack
    let (start, size) = (
        lds_address!(stack_start),
        lds_address!(stack_end) - lds_address!(stack_start),
    );
    map_area(
        VirtAddr::new(start),
        PhyAddr::from_virt(start),
        size,
        PTEFlags::RW,
        "stack",
    );
    //symbols
    let (start, size) = (
        lds_address!(symbols_start),
        lds_address!(symbols_end) - lds_address!(symbols_start),
    );
    map_area(
        VirtAddr::new(start),
        PhyAddr::from_virt(start),
        size,
        PTEFlags::R,
        "symbols",
    );
    //heap
    let (start, size) = (
        lds_address!(heap_start),
        MEM_SIZE - (lds_address!(heap_start) - lds_address!(text_start)),
    );
    map_area(
        VirtAddr::new(start),
        PhyAddr::from_virt(start),
        size,
        PTEFlags::RW,
        "heap",
    );
    pr_delimiter!();
    let kernel_space = KERNEL_SPACE.lock();
    let page_table_root = kernel_space.root_phy_addr();
    enable_table(page_table_root.as_usize(), true);
    enable_table(0, false);
}


#[no_mangle]
pub fn enable_table(page_table_root: usize, is_kernel: bool) {
    if is_kernel {
        // kernel space (0xffff_0000_0000_0000..0xffff_ffff_ffff_ffff)
        reg_write_p!(TTBR1_EL1, page_table_root)
    } else {
        // user space (0x0..0xffff_ffff_ffff)
        reg_write_p!(TTBR0_EL1, page_table_root)
    }
    isb_all();
    dsb_all();
    tlb_all();
}
