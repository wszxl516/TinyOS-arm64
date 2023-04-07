#ifndef __USER_LIB_H__
#define __USER_LIB_H__
#include "common.h"

#define SYSCALL(syscall_no)                 \
  do {                                      \
    __asm__ volatile("mov x8, " #syscall_no \
                     "\n"                   \
                     "svc 0");              \
  } while (0)

#endif  //__USER_LIB_H__