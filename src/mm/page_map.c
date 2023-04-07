#include "address.h"
#include "config.h"
#include "frame.h"
#include "map_page.h"
#include "mmu.h"
#include "printf.h"
#include "ptable.h"
#include "spinlock.h"
#include "stdtypes.h"
#include "string.h"

STATIC_INIT_SPIN_LOCK(page_lock);
static root_page_t ROOT_PAGE = {.look = &page_lock, .root_page = 0};

static inline phy_addr_t alloc_page() {
  phy_addr_t page = (phy_addr_t)frame_alloc_page();
  memset((void *)page, 0, PAGE_SIZE);
  return VIR_2_PHY(page);
}

bool map_page(phy_addr_t addr, mm_flags flags) {
  u32 l0 = PA_2_L0(addr), l1 = PA_2_L1(addr), l2 = PA_2_L2(addr),
      l3 = PA_2_L3(addr);
  if (!ROOT_PAGE.root_page) {
    phy_addr_t page = alloc_page();
    spin_lock(ROOT_PAGE.look);
    ROOT_PAGE.root_page = (phy_addr_t *)(page);
    spin_unlock(ROOT_PAGE.look);
  }
  phy_addr_t *pg0_addr = ROOT_PAGE.root_page;
  phy_addr_t *pg1_addr = (phy_addr_t *)(pg0_addr + l0);
  if (*pg1_addr == 0) {
    phy_addr_t page = alloc_page();
    spin_lock(ROOT_PAGE.look);
    *pg1_addr = (phy_addr_t)(page | MM_TYPE_PAGE_TABLE);
    spin_unlock(ROOT_PAGE.look);
  }
  phy_addr_t *pg2_addr = (phy_addr_t *)CLEAR_FLAG(*pg1_addr) + l1;
  if (*pg2_addr == 0) {
    phy_addr_t page = alloc_page();
    spin_lock(ROOT_PAGE.look);
    *pg2_addr = (phy_addr_t)(page | MM_TYPE_PAGE_TABLE);
    spin_unlock(ROOT_PAGE.look);
  }
  phy_addr_t *pg3_addr = (phy_addr_t *)CLEAR_FLAG(*pg2_addr) + l2;
  if (*pg3_addr == 0) {
    phy_addr_t page = alloc_page();
    spin_lock(ROOT_PAGE.look);
    *pg3_addr = (phy_addr_t)(page | MM_TYPE_PAGE_TABLE);
    spin_unlock(ROOT_PAGE.look);
  }
  // no check overwrite
  ((phy_addr_t *)CLEAR_FLAG(*pg3_addr))[l3] = addr | flags;
  return 0;
}

void kernel_map_range(phy_addr_t addr, usize size, mm_flags flags) {
  u32 num_pages = ALIGN_UP(size) / PAGE_SIZE;
  pr_table("[%0p - %0p]", 50, addr, addr + size);
  pr_table("[%0p - %0p]", 50, PHY_2_VIR(addr), PHY_2_VIR(addr) + size);
  for (u32 i = 0; i < num_pages; i++) {
    map_page(ALIGN_DOWN(addr) + i * PAGE_SIZE, flags);
  }
}

static void enable_kernel_page() {
  ENABLE_PAGE((usize)ROOT_PAGE.root_page, true);
  ENABLE_PAGE((usize)0, false);
}

void init_page_table() {
  // UART mapping
  map_page(UART_REGISTER_ADDR, KERNEL_RW_DATA | DEVICE_MEM);
  // GIC mapping
  map_page(GIC_BASE_ADDR, KERNEL_RW_DATA | DEVICE_MEM);
  // kernel rx text map
  pr_table("Map .text section", 50);
  kernel_map_range(VIR_2_PHY((usize)text_start),
                   (usize)bss_start - (usize)text_start,
                   KERNEL_RX_DATA | NORMAL_MEM);
  // kernel rw data map
  pr_table("Map .data .rodata .stack .. section", 50);
  kernel_map_range(VIR_2_PHY((usize)bss_start),
                   (usize)heap_start - (usize)bss_start,
                   KERNEL_RW_DATA | NORMAL_MEM);
  pr_table_end(50);
  enable_kernel_page();
}