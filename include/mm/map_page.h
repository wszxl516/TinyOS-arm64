#ifndef __MAP_PAGE_H__
#define __MAP_PAGE_H__
#include "address.h"
#include "arm64.h"
#include "mmu.h"
#include "spinlock.h"
#include "stdtypes.h"

typedef struct {
  phy_addr_t *root_page;
  spinlock_t *look;
} root_page_t;

typedef usize mm_flags;

#define ENABLE_PAGE(root_page, is_kernel)                                \
  do {                                                                   \
    if (is_kernel) {                                                     \
      REG_WRITE_P(TTBR1_EL1, root_page);                                 \
      REG_UPDATE_G(sp, KERNEL_VA_START);                                 \
    } else {                                                             \
      REG_WRITE_P(TTBR0_EL1, root_page);                                 \
    }                                                                    \
    REG_WRITE_P(tcr_el1, TCR_VALUE);                                     \
    REG_WRITE_P(mair_el1, MAIR_VALUE);                                   \
    REG_UPDATE_P(sctlr_el1, SCTLR_MMU_ENABLED | SCTLR_D_CACHE_DISABLED | \
                                SCTLR_I_CACHE_DISABLED);                 \
    ISB_ALL();                                                           \
    TLB_ALL();                                                           \
    DSB_ALL();                                                           \
  } while (0)

#define KERNEL_RX_DATA                                                  \
  (MM_FLAG_AF | MM_FLAG_VALID | MM_FLAG_NON_BLOCK | MM_FLAG_SHAREABLE | \
   MM_FLAG_INNER)
#define KERNEL_RW_DATA (KERNEL_RX_DATA | MM_FLAG_PXN)
#define KERNEL_RO_DATA (KERNEL_RW_DATA | MM_FLAG_AP_RO)

#define USER_RX_DATA \
  (MM_FLAG_VALID | MM_FLAG_AF | MM_FLAG_AP_EL0 | MM_FLAG_NON_BLOCK)
#define USER_RW_DATA (USER_RX_DATA | MM_FLAG_UXN)
#define USER_RO_DATA (USER_RW_DATA | MM_FLAG_AP_RO)

#define NORMAL_MEM (MM_FLAG_SHAREABLE | MM_FLAG_INNER)
#define DEVICE_MEM (0)

#define ALIGN_DOWN(addr) ((addr) & ~(PAGE_SIZE - 1))
#define ALIGN_UP(addr) (((addr) + PAGE_SIZE - 1) & ~(PAGE_SIZE - 1))

bool map_page(phy_addr_t addr, mm_flags flags);
void kernel_map_range(phy_addr_t addr, usize size, mm_flags flags);
void init_page_table();
#endif  //__MAP_PAGE_H__