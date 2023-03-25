#ifndef __MMU_H__
#define __MMU_H__
#include "common.h"
#include "address.h"

//https://developer.arm.com/documentation/ddi0595/2021-03/AArch64-Registers/MAIR-EL1--Memory-Attribute-Indirection-Register--EL1-
#define TCR_EPD1                                (0 << 23)          //Perform translation table walks using TTBR1_EL1.
#define TCR_SH0                                 (0 << 12)          //Shareability attribute for memory associated with translation table walks using TTBR0_EL1.
#define TCR_SH1                                 (0 << 28)          //Shareability attribute for memory associated with translation table walks using TTBR1_EL1.
#define TCR_T0SZ                                (64ULL - 48)           //2^16 B
#define TCR_T1SZ                                ((64 - 48) << 16)   //2^16 B
#define TCR_TG0_4K                              (0 << 14)
#define TCR_TG1_4K                              (2 << 30)
#define TCR_VALUE                               (TCR_T0SZ | TCR_T1SZ | TCR_TG0_4K | TCR_TG1_4K | TCR_EPD1 | TCR_SH0)
#define SCTLR_RESERVED                          (3ULL << 28) | (3ULL << 22) | (1ULL << 20) | (1ULL << 11)
#define SCTLR_EE_LITTLE_ENDIAN                  (0 << 25)
#define SCTLR_EOE_LITTLE_ENDIAN                 (0 << 24)
#define SCTLR_I_CACHE_DISABLED                  (0 << 12)
#define SCTLR_D_CACHE_DISABLED                  (0 << 2)
#define SCTLR_MMU_DISABLED                      (0 << 0)
#define SCTLR_MMU_ENABLED                       (1 << 0)

#define SCTLR_VALUE_MMU_DISABLED                (SCTLR_RESERVED | SCTLR_EE_LITTLE_ENDIAN | SCTLR_I_CACHE_DISABLED | SCTLR_D_CACHE_DISABLED | SCTLR_MMU_DISABLED)

#define MM_TYPE_PAGE_TABLE                      (0x3)
#define MEM_BASE                                (0x40000000ULL)                    // DRAM BASE
#define MEM_SIZE                                (0x8000000ULL)
#define MMU_L2_SIZE                             (0x200000ULL)
#define MM_ACCESS                               (0x1 << 10)
#define MM_TYPE_BLOCK                           0x1ULL
#define MT_DEVICE_nGnRnE                        0x0ULL
#define MT_NORMAL_NC                            0x1ULL
#define MT_DEVICE_nGnRnE_FLAGS                  0x00ULL
#define MT_NORMAL_NC_FLAGS                      0x44ULL
#define TABLE_SHIFT                             (9)
//The linear mapping and the start of memory are both 2M aligned
#define SECTION_SHIFT                           (PAGE_SHIFT + TABLE_SHIFT)
#define SECTION_SIZE                            (1 << SECTION_SHIFT)   


#define MAIR_VALUE                              (MT_DEVICE_nGnRnE_FLAGS << (8 * MT_DEVICE_nGnRnE)) | (MT_NORMAL_NC_FLAGS << (8 * MT_NORMAL_NC))

#define MMU_NORMAL_FLAGS                        (MM_ACCESS | (MT_NORMAL_NC << 2)     | MM_TYPE_BLOCK)   
#define MMU_DEVICE_FLAGS                        (MM_ACCESS | (MT_DEVICE_nGnRnE << 2) | MM_TYPE_BLOCK)   
#define MEM_TYPE(flags)                         ((0b100 & flags) ? "NORMAL": "DEVICE")

//Instruction Synchronization Barrier
#define ISB_ALL()   ({__asm__ volatile("ISB SY");})
//The Translation Lookaside Buffer
#define TLB_ALL()   ({__asm__ volatile("tlbi vmalle1is");})
//Data Memory Barrier
#define DSB_ALL()   ({ __asm__ volatile("DSB SY") ;})

void setup_mmu();
#endif //__MMU_H__