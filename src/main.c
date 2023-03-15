#include "uart.h"
#include "printf.h"
#include "common.h"

void FUNC_NORETURN kernel_main(){
    init_uart();
    pr_info("Started!\n");
    char buffer[16] = {0};
    while (1){
        gets(buffer, 16);
        pr_info("buffer: %s\n", buffer);
    }
}