#include "arm64.h"
#include "ptable.h"
#include "common.h"
#include "config.h"


void setup_trap()
{
    pr_table("Setup Traps!", 50);
    pr_table_end(50);
    usize trap_address = (usize)trap_vector;
    REG_WRITE_P(VBAR_EL1, trap_address);
}


