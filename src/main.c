#include "common.h"
#include "stdtypes.h"
#include "uart.h"
#include "printf.h"
#include "feature.h"
#include "arm64.h"
#include "timer.h"

extern void FUNC_NORETURN test_user();

void FUNC_NORETURN kernel_main(){
    init_uart();
    feature_dump();
    timer_init();
    enable_irq();
    pr_info("Current EL: %u\n\n", CURRENT_EL());
    pr_info("Switch to EL0\n");
    SWITCH_TO_USER(test_user);
    while (true);
}