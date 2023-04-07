
#ifndef __CPU_H__
#define __CPU_H__
#include "config.h"
#include "stdtypes.h"
// read generic register
#define REG_READ_G(name)                             \
  ({                                                 \
    usize value = 0;                                 \
    __asm__ volatile("mov %0," #name : "=r"(value)); \
    value;                                           \
  })

// write generic register
#define REG_WRITE_G(name, value) \
  ({ __asm__ volatile("mov " #name ", %0" : : "r"(value) : "memory"); })

// read Privileged register
#define REG_READ_P(name)                                              \
  ({                                                                  \
    usize value = 0;                                                  \
    __asm__ __volatile__("mrs %0," #name : "=r"(value) : : "memory"); \
    value;                                                            \
  })

// write Privileged register
#define REG_WRITE_P(name, value) \
  ({ __asm__ volatile("msr " #name ", %0" : : "r"(value)); })

#define REG_UPDATE_P(name, value) REG_WRITE_P(name, (REG_READ_P(name) | value))
#define REG_UPDATE_G(name, value) REG_WRITE_G(name, (REG_READ_G(name) | value))

#define CURRENT_EL() GET_BITS(REG_READ_P(CurrentEL), 2, 3)

#define SWITCH_TO_USER(func)           \
  do {                                 \
    REG_WRITE_P(spsr_el1, 0ULL);       \
    REG_WRITE_P(elr_el1, (usize)func); \
    __asm__ volatile("eret");          \
  } while (0)

#endif  //__CPU_H__