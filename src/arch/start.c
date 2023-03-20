#include "arm64.h"
#include "printf.h"
#include "common.h"
#include "config.h"
#include "mmu.h"

void setup_trap()
{
    pr_notice("Setup traps!\n");
    usize trap_address = (usize)trap_vector;
    REG_WRITE_P(VBAR_EL1, trap_address);
}


static usize SECTION(".pagetable") pg_tbl1[512] = {0};
static usize SECTION(".pagetable") pg_tbl2[512] = {0};
static usize SECTION(".pagetable") pg_tbl3[512] = {0};
static usize SECTION(".pagetable") ram_tbl1[512] = {0};
static usize SECTION(".pagetable") ram_tbl2[512] = {0};
static usize SECTION(".pagetable") ram_tbl3[512] = {0};

void setup_mmu(){
    pr_notice("Setup mmu!\n");
    REG_WRITE_P(sctlr_el1, SCTLR_VALUE_MMU_DISABLED);
    pg_tbl1[0] = (usize)pg_tbl2 | MM_TYPE_PAGE_TABLE;
    pg_tbl2[0] = (usize)pg_tbl3 | MM_TYPE_PAGE_TABLE;


    usize  base_addr = MEM_BASE;
    u32 i = 0;
    for (; i < 510; i++)
    {
        pg_tbl3[i] = base_addr | MMU_FLAGS;
        base_addr += SECTION_SIZE;
    }
    pg_tbl3[i] = UART_REGISTER_ADDR | MMU_DEVICE_FLAGS;

    /*@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@*/
    ram_tbl1[0] = (usize)ram_tbl2 | MM_TYPE_PAGE_TABLE;
    ram_tbl2[1] = (usize)ram_tbl3 | MM_TYPE_PAGE_TABLE;
    //map 0x40000000 2MB
    ram_tbl3[0] = MEM_BASE | MMU_FLAGS;

    /*@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@*/
    REG_WRITE_P(ttbr1_el1, (usize)pg_tbl1);
    REG_WRITE_P(ttbr0_el1, (usize)ram_tbl1);
    REG_WRITE_P(tcr_el1, TCR_VALUE);
    REG_WRITE_P(mair_el1, MAIR_VALUE);
    REG_UPDATE_P(sctlr_el1, 1 | 1 << 2 | 1 << 12);
    ISB_ALL();
    TLB_ALL();
    DSB_ALL();
    return;
}
