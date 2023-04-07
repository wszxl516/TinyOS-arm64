#include "mmu.h"

#include "arm64.h"
#include "config.h"
#include "ptable.h"
#include "stdtypes.h"

static usize SECTION(".pagetable") pg_tbl0[512] = {0};
static usize SECTION(".pagetable") pg_tbl1[512] = {0};
static usize SECTION(".pagetable") pg_tbl2_0[512] = {0};
static usize SECTION(".pagetable") pg_tbl2_1[512] = {0};

static void early_map(usize phy_addr, usize flags) {
  u32 p0 = PA_2_L0(phy_addr), p1 = PA_2_L1(phy_addr), p2 = PA_2_L2(phy_addr);
  usize *pg1 = GET_PAGE_ADDR((usize)(&pg_tbl0[p0]));
  usize *pg2 = (usize *)CLEAR_FLAG(*(pg1 + p1));
  *(pg2 + p2) = phy_addr | flags;
}

static void early_map_parts(usize start, usize size, usize flags) {
  for (u32 i = 0; i < size / SECTION_SIZE; i++)
    early_map(start + SECTION_SIZE * i, flags);
}

static void init_mmu() {
  REG_WRITE_P(sctlr_el1, SCTLR_VALUE_MMU_DISABLED);
  pg_tbl0[0] = (usize)pg_tbl1 | MM_TYPE_PAGE_TABLE;
  pg_tbl1[0] = (usize)pg_tbl2_0 | MM_TYPE_PAGE_TABLE;
  pg_tbl1[1] = (usize)pg_tbl2_1 | MM_TYPE_PAGE_TABLE;
}

static void enable_mmu(void *ttbr0, void *ttbr1) {
  REG_WRITE_P(ttbr1_el1, (usize)ttbr0);
  REG_WRITE_P(ttbr0_el1, (usize)ttbr1);
  REG_WRITE_P(tcr_el1, TCR_VALUE);
  REG_WRITE_P(mair_el1, MAIR_VALUE);
  REG_UPDATE_P(sctlr_el1, SCTLR_MMU_ENABLED | SCTLR_D_CACHE_DISABLED |
                              SCTLR_I_CACHE_DISABLED);
  ISB_ALL();
  TLB_ALL();
  DSB_ALL();
}

void setup_mmu() {
  pr_table("MMU early map", 50);
  pr_table_end(50);
  init_mmu();
  // map 0x40000000 size 2MB
  // The linear mapping and the start of memory are both 2M aligned
  early_map_parts(MEM_BASE, MEM_SIZE, MMU_NORMAL_FLAGS);
  // map 0x09000000 for uart
  early_map_parts(UART_REGISTER_ADDR, MMU_L2_SIZE, MMU_DEVICE_FLAGS);
  early_map_parts(GIC_BASE_ADDR, MMU_L2_SIZE, MMU_DEVICE_FLAGS);
  /*@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@*/
  enable_mmu(pg_tbl0, pg_tbl0);
  return;
}