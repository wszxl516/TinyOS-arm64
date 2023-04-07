#include "address.h"
#include "arm64.h"
#include "common.h"
#include "config.h"
#include "feature.h"
#include "frame.h"
#include "map_page.h"
#include "mmu.h"
#include "printf.h"
#include "ptable.h"
#include "stdtypes.h"
#include "timer.h"
#include "uart.h"

extern void FUNC_NORETURN test_user();

void FUNC_NORETURN kernel_main() {
  init_uart();
  feature_dump();
  timer_init();
  enable_irq();
  frma_alloc_init();
  init_page_table();
  pr_table("Current EL: %u", 50, CURRENT_EL());
  pr_table("Switch to EL0", 50);
  pr_table_end(50);
  SWITCH_TO_USER(test_user);
  while (true)
    ;
}