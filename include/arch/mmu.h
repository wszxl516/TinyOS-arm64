#ifndef __MMU_H__
#define __MMU_H__
#include "address.h"
#include "common.h"
#include "config.h"

// https://developer.arm.com/documentation/ddi0595/2021-03/AArch64-Registers/MAIR-EL1--Memory-Attribute-Indirection-Register--EL1-
#define TCR_EPD1 (0 << 23)  // Perform translation table walks using TTBR1_EL1.
#define TCR_SH0 \
  (0 << 12)  // Shareability attribute for memory associated with translation
             // table walks using TTBR0_EL1.
#define TCR_SH1 \
  (0 << 28)  // Shareability attribute for memory associated with translation
             // table walks using TTBR1_EL1.
#define TCR_T0SZ (64ULL - 48)       // 2^16 B
#define TCR_T1SZ ((64 - 48) << 16)  // 2^16 B
#define TCR_TG0_4K (0 << 14)
#define TCR_TG1_4K (2 << 30)
#define TCR_VALUE \
  (TCR_T0SZ | TCR_T1SZ | TCR_TG0_4K | TCR_TG1_4K | TCR_EPD1 | TCR_SH0)
#define SCTLR_RESERVED (3ULL << 28) | (3ULL << 22) | (1ULL << 20) | (1ULL << 11)
#define SCTLR_EE_LITTLE_ENDIAN (0 << 25)
#define SCTLR_EOE_LITTLE_ENDIAN (0 << 24)
#define SCTLR_I_CACHE_DISABLED (0 << 12)
#define SCTLR_D_CACHE_DISABLED (0 << 2)
#define SCTLR_MMU_DISABLED (0 << 0)
#define SCTLR_MMU_ENABLED (1 << 0)
#define SCTLR_VALUE_MMU_DISABLED                                      \
  (SCTLR_RESERVED | SCTLR_EE_LITTLE_ENDIAN | SCTLR_I_CACHE_DISABLED | \
   SCTLR_D_CACHE_DISABLED | SCTLR_MMU_DISABLED)

/// Whether the descriptor is valid.
#define MM_FLAG_VALID (1ULL << 0)
/// The descriptor gives the address of the next level of translation table or
/// 4KB page. (not a 2M, 1G block)
#define MM_FLAG_NON_BLOCK (1ULL << 1)
/// Memory attributes index field.
#define MM_FLAG_ATTR_INDX (0b111 << 2)
/// Non-secure bit. For memory accesses from Secure state, specifies whether the
/// output address is in Secure or Non-secure memory.
#define MM_FLAG_NS (1ULL << 5)
/// Access permission: accessable at EL0.
#define MM_FLAG_AP_EL0 (1ULL << 6)
/// Access permission: read-only.
#define MM_FLAG_AP_RO (1ULL << 7)
/// Shareability: Inner Shareable (otherwise Outer Shareable).
#define MM_FLAG_INNER (1ULL << 8)
/// Shareability: Inner or Outer Shareable (otherwise Non-shareable).
#define MM_FLAG_SHAREABLE (1ULL << 9)
/// The Access flag.
#define MM_FLAG_AF (1ULL << 10)
/// The not global bit.
#define MM_FLAG_NG (1ULL << 11)
/// Indicates that 16 adjacent translation table entries point to contiguous
/// memory regions.
#define MM_FLAG_CONTIGUOUS (1ULL << 52)
/// The Privileged execute-never field.
#define MM_FLAG_PXN (1ULL << 53)
/// The Execute-never or Unprivileged execute-never field.
#define MM_FLAG_UXN (1ULL << 54)

// Next-level attributes in stage 1 VMSAv8-64 Table descriptors:

/// PXN limit for subsequent levels of lookup.
#define MM_FLAG_PXN_TABLE (1ULL << 59)
/// XN limit for subsequent levels of lookup.
#define MM_FLAG_XN_TABLE (1ULL << 60)
/// Access permissions limit for subsequent levels of lookup: access at EL0 not
/// permitted.
#define MM_FLAG_AP_NO_EL0_TABLE (1ULL << 61)
/// Access permissions limit for subsequent levels of lookup: write access not
/// permitted.
#define MM_FLAG_AP_NO_WRITE_TABLE (1ULL << 62)
/// For memory accesses from Secure state, specifies the Security state for
/// subsequent levels of lookup.
#define MM_FLAG_NS_TABLE (1ULL << 63)
#define MM_TYPE_PAGE_TABLE (0x3)
#define MMU_L2_SIZE (0x200000ULL)
// The linear mapping and the start of memory are both 2M aligned
#define SECTION_SIZE (MMU_L2_SIZE)

#define MT_DEVICE_nGnRnE 0x0ULL
#define MT_NORMAL_NC 0x1ULL
#define MT_DEVICE_nGnRnE_FLAGS 0x00ULL
#define MT_NORMAL_NC_FLAGS 0x44ULL
#define MAIR_VALUE                                     \
  (MT_DEVICE_nGnRnE_FLAGS << (8 * MT_DEVICE_nGnRnE)) | \
      (MT_NORMAL_NC_FLAGS << (8 * MT_NORMAL_NC))

#define MMU_NORMAL_FLAGS (MM_FLAG_AF | (MT_NORMAL_NC << 2) | MM_FLAG_VALID)
#define MMU_DEVICE_FLAGS (MM_FLAG_AF | (MT_DEVICE_nGnRnE << 2) | MM_FLAG_VALID)
#define MEM_TYPE(flags) ((0b100 & flags) ? "NORMAL" : "DEVICE")

// Instruction Synchronization Barrier
#define ISB_ALL() ({ __asm__ volatile("ISB SY"); })
// The Translation Lookaside Buffer
#define TLB_ALL() ({ __asm__ volatile("tlbi vmalle1is"); })
// Data Memory Barrier
#define DSB_ALL() ({ __asm__ volatile("DSB SY"); })

void setup_mmu();
#endif  //__MMU_H__