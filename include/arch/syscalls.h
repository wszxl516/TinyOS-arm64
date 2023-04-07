#ifndef __SYSCALL_H__
#define __SYSCALL_H__
#include "common.h"

typedef struct {
  usize args[6];
  usize syscall_nr;
} syscall_args_t;

#define SYSCALL_HELLO (0)
#define SYSCALL_WORLD (1)
#define SYSCALL_PAUSE (2)

isize do_syscall(syscall_args_t *args);
#endif  //__SYSCALL_H__