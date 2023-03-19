#include "arm64.h"
#include "printf.h"
#include "common.h"

void dump_error(void *stack_address){
    pr_err("Context stack addr: %p\n", stack_address);
    usize value = REG_READ_P(ESR_EL1);
    pr_err("ISS: 0x%x 0x%x IL: 0x%x  EC: 0x%x\n", 
            GET_BITS(value, 0, 24),
            GET_BITS(value, 32, 36),
            GET_BIT(value, 25),
            GET_BITS(value, 26, 31)
            );
    while (true);
}