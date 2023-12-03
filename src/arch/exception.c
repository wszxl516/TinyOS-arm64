#include "exception.h"

#include "arm64.h"
#include "common.h"
#include "gic.h"
#include "printf.h"
#include "symbol.h"
#include "syscalls.h"
#include "timer.h"

void dump_trap_fram(trap_frame *frame) {
  char name[64] = {0};
  usize offset = 0;
  for (u32 i = 0; i < 31; i++)
  {
    if (i % 3 == 0)
      pr_err("\n");
    pr_err("x%02u %0p\t", i, frame->regs[i]);
  }
  pr_err("\n");
  pr_err("ELR_EL1: %0p\t", frame->elr);
  pr_err("SPSR_EL1: %0p\n", frame->spsr);
  lookup_name_and_offset(frame->regs[30], name, &offset);
  pr_err("#(%0p):%s+0x%x\n", frame->regs[30], name, offset);
  lookup_name_and_offset(frame->elr, name, &offset);
  pr_err("\t#(%0p):%s+0x%x\n", frame->elr, name, offset);
}

void dump_error(trap_frame *frame) {
  usize value = REG_READ_P(ESR_EL1);
  pr_err("\n\n%s\n", exception_msg[GET_BITS(value, 26, 31)]);
  pr_err("ISS: 0x%x 0x%x IL: 0x%x  EC: 0x%x\n", GET_BITS(value, 0, 24),
         GET_BITS(value, 32, 36), GET_BIT(value, 25), GET_BITS(value, 26, 31));
  dump_trap_fram(frame);
  while (true)
    ;
}

void system_call(trap_frame *frame) {
  syscall_args_t args = {
      .args = {frame->regs[0], frame->regs[1], frame->regs[2], frame->regs[3],
               frame->regs[4], frame->regs[5]},
      .syscall_nr = frame->regs[8]};
  frame->regs[0] = do_syscall(&args);
}

void handle_irq(trap_frame *frame __UNUSED__) {
  disable_irq();
  i32 irq;
  if (gic_fetch_irq(&irq) == IRQ_NOT_FOUND) {
    usize value = REG_READ_P(ESR_EL1);
    switch (GET_BITS(value, 26, 31)) {
      case EC_SYSCALL:
        system_call(frame);
        break;
      default:
        dump_error(frame);
    }
  }

  switch (irq) {
    case TIMER_IRQ:
      timer_handler();
      break;
    default:
      break;
  }
  enable_irq();
}