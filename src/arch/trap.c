#include "arm64.h"
#include "printf.h"
#include "common.h"
#include "config.h"


void setup_trap()
{
    pr_info("#####################################\n");
    pr_notice("Setup traps!\n");
    usize trap_address = (usize)trap_vector;
    REG_WRITE_P(VBAR_EL1, trap_address);
}


