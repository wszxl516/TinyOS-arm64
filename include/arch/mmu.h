#ifndef __MMU_H__
#define __MMU_H__
#include "common.h"
#define KERNELE_SPACE_START     0XFFFF000000000000
#define KERNELE_SPACE_END       0XFFFFFFFFFFFFFFFF

#define USER_SPACE_START        0X0000000000000000
#define USER_SPACE_END          0X0000FFFFFFFFFFFF



//https://developer.arm.com/documentation/ddi0595/2021-03/AArch64-Registers/MAIR-EL1--Memory-Attribute-Indirection-Register--EL1-
#define TCR_T0SZ                                (64 - 48)           //2^16 B
#define TCR_T1SZ                                ((64 - 48) << 16)   //2^16 B
#define TCR_TG0_4K                              (0 << 14)
#define TCR_TG1_4K                              (2 << 30)
#define TCR_VALUE                               (TCR_T0SZ | TCR_T1SZ | TCR_TG0_4K | TCR_TG1_4K)
#define SCTLR_RESERVED                          (3 << 28) | (3 << 22) | (1 << 20) | (1 << 11)
#define SCTLR_EE_LITTLE_ENDIAN                  (0 << 25)
#define SCTLR_EOE_LITTLE_ENDIAN                 (0 << 24)
#define SCTLR_I_CACHE_DISABLED                  (0 << 12)
#define SCTLR_D_CACHE_DISABLED                  (0 << 2)
#define SCTLR_MMU_DISABLED                      (0 << 0)
#define SCTLR_MMU_ENABLED                       (1 << 0)

#define SCTLR_VALUE_MMU_DISABLED                (SCTLR_RESERVED | SCTLR_EE_LITTLE_ENDIAN | SCTLR_I_CACHE_DISABLED | SCTLR_D_CACHE_DISABLED | SCTLR_MMU_DISABLED)
#define KERNEL_VA_START                         (KERNELE_SPACE_START)
#define PGD_SHIFT                               (39)
#define PUD_SHIFT                               (30)
#define PMD_SHIFT                               (21)
#define PTE_SHIFT                               (12)
#define PAGE_SHIFT                              (12)
#define PAGE_SIZE                               (1 << PAGE_SHIFT)
#define MM_TYPE_PAGE_TABLE                      0x3
#define MEM_BASE                                (0x40000000)                    // DRAM BASE
#define MM_ACCESS                               (0x1 << 10)
#define MM_TYPE_BLOCK                           0x1
#define MT_DEVICE_nGnRnE                        0x0
#define MT_NORMAL_NC                            0x1
#define MT_DEVICE_nGnRnE_FLAGS                  0x00
#define MT_NORMAL_NC_FLAGS                      0x44
#define TABLE_SHIFT                             (9)
#define MMU_FLAGS                               (MM_ACCESS | (MT_NORMAL_NC << 2) | MM_TYPE_BLOCK)  
#define SECTION_SHIFT                           (PAGE_SHIFT + TABLE_SHIFT)
#define SECTION_SIZE                            (1 << SECTION_SHIFT)   


#define MAIR_VALUE                              (MT_DEVICE_nGnRnE_FLAGS << (8 * MT_DEVICE_nGnRnE)) | (MT_NORMAL_NC_FLAGS << (8 * MT_NORMAL_NC))

#define MMU_FLAGS                               (MM_ACCESS | (MT_NORMAL_NC << 2) | MM_TYPE_BLOCK)   
#define MMU_DEVICE_FLAGS                        (MM_ACCESS | (MT_DEVICE_nGnRnE << 2) | MM_TYPE_BLOCK)   

#define PG_NUM_TO_VIRT(pgd, pud ,pmd)           ((pgd << PGD_SHIFT) | (pud << PUD_SHIFT) | (pmd << PMD_SHIFT)  | KERNEL_VA_START)
#define VIRT_UART_BASE                          PG_NUM_TO_VIRT(0LLU, 0LLU, 510LLU)
//Instruction Synchronization Barrier
#define ISB_ALL()   ({__asm__ volatile("ISB SY");})
//The Translation Lookaside Buffer
#define TLB_ALL()   ({__asm__ volatile("tlbi vmalle1is");})
//Data Memory Barrier
#define DSB_ALL()   ({ __asm__ volatile("DSB SY") ;})
#endif //__MMU_H__