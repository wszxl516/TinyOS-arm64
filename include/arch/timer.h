#ifndef __TIMER_H__
#define  __TIMER_H__ 
#include "arm64.h"
#include "gic.h"
#include "stdtypes.h"

#define CNTV_CTL_ENABLE		(1 << 0)	/* Enables the timer */	
#define TIMER_PERIOD_MS     10

/* Counter-timer Virtual Timer CompareValue register*/
#define READ_COUNTER_TIMER_CMP() 		REG_READ_P(CNTV_CVAL_EL0)
#define WRITE_COUNTER_TIMER_CMP(value)	REG_WRITE_P(CNTV_CVAL_EL0, value)

/* Counter-timer Virtual Count register*/
#define READ_COUNTER_TIMER()			REG_READ_P(CNTVCT_EL0)

/*Counter-timer Virtual Timer Control register*/
#define  DISABLE_TIMER()				REG_WRITE_P(CNTV_CTL_EL0, REG_READ_P(CNTV_CTL_EL0) & ~CNTV_CTL_ENABLE)
#define  ENABLE_TIMER()					REG_WRITE_P(CNTV_CTL_EL0, REG_READ_P(CNTV_CTL_EL0) | CNTV_CTL_ENABLE);

/*CNTFRQ_EL0, Counter-timer Frequency register*/
#define  READ_FREQUENCY()				REG_READ_P(CNTFRQ_EL0)
#define  WRITE_FREQUENCY(value)			REG_WRITE_P(CNTFRQ_EL0, value)

extern void timer_init(void);
extern void timer_handler(void);
#endif  /* __TIMER_H__  */