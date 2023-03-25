#ifndef __ADDRESS_H__
#define __ADDRESS_H__
#include "common.h"
#include "config.h"

#define KERNELE_SPACE_START                     (0xffff000000000000ULL)
#define KERNELE_SPACE_END                       (0xffffffffffffffffULL)

#define USER_SPACE_START                        (0X0000000000000000ULL)
#define USER_SPACE_END                          (0x0000ffffffffffffULL)
#define KERNEL_VA_START                         (KERNELE_SPACE_START)

#define PGD_SHIFT                               (39)
#define PUD_SHIFT                               (30)
#define PMD_SHIFT                               (21)
#define PTE_SHIFT                               (12)
#define PAGE_SHIFT                              (12)
#define PAGE_SIZE                               (1 << PAGE_SHIFT)
#define PAGE_MASK                               ((1<<9) -1)


#define PHY_2_VIR(addr)                         ((addr) | (KERNEL_VA_START))
#define PA_2_L0(pa)                             ((pa >> PGD_SHIFT ) & PAGE_MASK)
#define PA_2_L1(pa)                             ((pa >> PUD_SHIFT ) & PAGE_MASK)
#define PA_2_L2(pa)                             ((pa >> PMD_SHIFT ) & PAGE_MASK)
#define PA_2_L3(pa)                             ((pa >> PTE_SHIFT ) & PAGE_MASK)
#define CLEAR_FLAG(addr)                        ((addr >> PTE_SHIFT) <<PTE_SHIFT)
#define GET_PAGE_ADDR(addr)                     ((usize*)(CLEAR_FLAG(*((usize*)CLEAR_FLAG(addr)))))
#define PG_NUM_TO_VIRT(l0, l1 ,l2, l3)          ((l0 << PGD_SHIFT) | (l1 << PUD_SHIFT) | (l2 << PMD_SHIFT) | (l3 << PTE_SHIFT) | KERNEL_VA_START)

typedef usize phy_addr_t;
typedef usize vir_addr_t;

#endif //__ADDRESS_H__