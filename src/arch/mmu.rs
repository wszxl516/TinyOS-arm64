use crate::mm::{attr::PTEFlags, entry::PTE};
use crate::mm::address::PhyAddr;
use crate::mm::flush::{dsb_all, isb_all, tlb_all};
use crate::mm::page::PageTable;
use crate::reg_write_p;

use super::reg::{MAIR_EL1, SCTLR_EL1, TCR_EL1};

#[link_section = ".data.boot_page_table"]
static mut BOOT_PT_0: PageTable = PageTable::new();

#[link_section = ".data.boot_page_table"]
static mut BOOT_PT_1: [PTE; 2] = [PTE::empty(); 2];

#[no_mangle]
pub fn init_mmu() {
    unsafe {
        // 0x0000_0000_0000 ~ 0x0080_0000_0000, table
        BOOT_PT_0.entrys[0] = PTE::new_page(PhyAddr::new(BOOT_PT_1.as_ptr() as usize));
        // 0x0000_0000_0000..0x0000_4000_0000, block, device memory
        // 1GB
        BOOT_PT_1[0] = PTE::new_entry(
            PhyAddr::new(0),
            PTEFlags::RW | PTEFlags::DEVICE,
            true,
        );
        // 0x0000_4000_0000..0x0000_8000_0000, block, normal memory
        // 1GB
        BOOT_PT_1[1] = PTE::new_entry(
            PhyAddr::new(0x4000_0000),
            PTEFlags::RWX,
            true,
        );
    }

    // setup memory attributes in MAIR
    MAIR_EL1::write(
        MAIR_EL1::attr(1, MAIR_EL1::device::nGnRE)
            | MAIR_EL1::attr(0, MAIR_EL1::normal::inner::WriteBackNonTransient
            | MAIR_EL1::normal::inner::ReadAllocate | MAIR_EL1::normal::inner::WriteAllocate
            | MAIR_EL1::normal::outer::WriteBackNonTransient
            | MAIR_EL1::normal::outer::ReadAllocate | MAIR_EL1::normal::outer::WriteAllocate,
        ),
    );
    // setup translation controls
    TCR_EL1::write(
        TCR_EL1::BITS40_1TB
            | TCR_EL1::T1SZ(16)
            | TCR_EL1::WALKS_ON_MISS
            | TCR_EL1::IRGN1_NORMAL_INNER
            | TCR_EL1::ORGN1_NORMAL_OUTER
            | TCR_EL1::SH1_INNER_SHAREABLE
            | TCR_EL1::TG1_4KB
            | TCR_EL1::T0SZ(16)
            // | TCR_EL1::TTBR0_WALKS_ON_MISS
            | TCR_EL1::IRGN0_NORMAL_INNER
            | TCR_EL1::ORGN0_NORMAL_OUTER
            | TCR_EL1::SH0_INNER_SHAREABLE
            | TCR_EL1::TG0_4KB
    );
    let root_addr = unsafe { core::ptr::addr_of!(BOOT_PT_0).addr() };
    // Set both TTBR0 and TTBR1
    reg_write_p!(TTBR0_EL1, root_addr);
    reg_write_p!(TTBR1_EL1, root_addr);

    // enable icache, dcache, sp check
    SCTLR_EL1::write(
        SCTLR_EL1::M
            | SCTLR_EL1::C
            | SCTLR_EL1::I
    );

    isb_all();
    dsb_all();
    tlb_all();
}
