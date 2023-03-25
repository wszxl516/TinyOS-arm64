#include "timer.h"
#include "printf.h"
#include "stdtypes.h"


static usize jiffies = 0;
static u32 ticks;

void timer_handler(void)
{
	// Disable the timer
	DISABLE_TIMER();
	gicd_clear_pending(TIMER_IRQ);
	// Set the interrupt in Current Time + TimerTick
	WRITE_COUNTER_TIMER_CMP(READ_COUNTER_TIMER() + ticks);
    jiffies++;
	ENABLE_TIMER();
}

void timer_init(void)
{
	// GIC Init
	gic_init();

	// Disable the timer
	DISABLE_TIMER();
	// Next timer IRQ is after n ms.
	ticks = TIMER_PERIOD_MS * (READ_FREQUENCY() / 1000);  //1000 for ms in s
	// Set the interrupt in Current Time + TimerTick
	WRITE_COUNTER_TIMER_CMP(READ_COUNTER_TIMER() + ticks);
	ENABLE_TIMER();
}

