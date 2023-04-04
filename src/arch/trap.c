#include "arm64.h"
#include "ptable.h"
#include "common.h"
#include "config.h"
#include "stdtypes.h"

void disable_irq(void)
{
	__asm__ __volatile__("msr DAIFSet, %0\n\t" : : "i" (1 << 1)  : "memory");
}

void enable_irq(void)
{
        	        u32 x __USED__ = *((usize *)(1<<64));

	__asm__ __volatile__("msr DAIFClr, %0\n\t" : : "i" (1 << 1)  : "memory");

}

void setup_trap()
{
    pr_table("Setup Traps!", 50);
    pr_table_end(50);
    usize trap_address = (usize)trap_vector;
    REG_WRITE_P(VBAR_EL1, trap_address);
}


