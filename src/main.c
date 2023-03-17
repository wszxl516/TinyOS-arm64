#include "uart.h"
#include "printf.h"
#include "feature.h"
#include "arm64.h"

void FUNC_NORETURN kernel_main(){
    init_uart();
    feature_dump();
    pr_info("Current EL: %u\n\n", CURRENT_EL());
    char buffer[32] = {0};
    char prompt[] = ">> ";
    pr_info("%s", prompt);
    while (1){
        gets(buffer, 16);
        pr_info("buffer: %s\n", buffer);
        pr_info("%s", prompt);
    }
}