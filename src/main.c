#include "uart.h"
#include "printf.h"
#include "common.h"
#include "feature.h"

void FUNC_NORETURN kernel_main(){
    init_uart();
    processor_feature();
    char buffer[16] = {0};
    while (1){
        gets(buffer, 16);
        pr_info("buffer: %s\n", buffer);
    }
}