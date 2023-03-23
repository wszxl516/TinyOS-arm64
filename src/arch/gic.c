#include "gic.h"
static GICC_REG *GICC_REGISTER = (GICC_REG*)GIC_GICC_BASE;
static GICD_REG *GICD_REGISTER = (GICD_REG*)GIC_GICD_BASE;

/* Initialize GIC Controller */
static void init_gicc(void)
{
	u32 pending_irq;

	/* Disable CPU interface */
	GICC_REGISTER->GIC_GICC_CTLR = GICC_CTLR_DISABLE;

	/* Set the priority level as the lowest priority */
	/* Note: Higher priority corresponds to a lower Priority field value in the GIC_PMR.
	 * In addition to this, writing 255 to the GICC_PMR always sets it to the 
	 * largest supported priority field value.
	 */
	GICC_REGISTER->GIC_GICC_PMR = GICC_PMR_PRIO_MIN;

	/* Handle all of interrupts in a single group */
	GICC_REGISTER->GIC_GICC_BPR = GICC_BPR_NO_GROUP;

	/* Clear all of the active interrupts */
	for(pending_irq = ( GICC_REGISTER->GIC_GICC_IAR & GICC_IAR_INTR_IDMASK ); 
	    ( pending_irq != GICC_IAR_SPURIOUS_INTR );
	    pending_irq = ( GICC_REGISTER->GIC_GICC_IAR & GICC_IAR_INTR_IDMASK ) )
		GICC_REGISTER->GIC_GICC_EOIR = GICC_REGISTER->GIC_GICC_IAR;

	/* Enable CPU interface */
	GICC_REGISTER->GIC_GICC_CTLR = GICC_CTLR_ENABLE;
}

static void init_gicd(void)
{
	i32	i, regs_nr;

	/* Diable distributor */
	GICD_REGISTER->GIC_GICD_CTLR = GIC_GICD_CTLR_DISABLE;

	/* Disable all IRQs */
	regs_nr = (GIC_INT_MAX + GIC_GICD_INT_PER_REG - 1) / GIC_GICD_INT_PER_REG;
	for (i = 0; regs_nr > i; ++i)
		GICD_REGISTER->GIC_GICD_ICENABLER[i] = ~((u32)(0)); 

	/* Clear all pending IRQs */
	regs_nr = (GIC_INT_MAX + GIC_GICD_INT_PER_REG - 1) / GIC_GICD_INT_PER_REG;
	for (i = 0; regs_nr > i; ++i) 
		GICD_REGISTER->GIC_GICD_ICPENDR[i] = ~((u32)(0));

	/* Set all of interrupt priorities as the lowest priority */
	regs_nr = ( GIC_INT_MAX + GIC_GICD_IPRIORITY_PER_REG - 1) / 
		GIC_GICD_IPRIORITY_PER_REG ;
	for (i = 0; regs_nr > i; i++)
		GICD_REGISTER->GIC_GICD_IPRIORITYR[i] = ~((u32)(0));

	/* Set target of all of shared peripherals to processor 0 */
	for (i = GIC_INTNO_SPI0 / GIC_GICD_ITARGETSR_PER_REG;
	     ( (GIC_INT_MAX + (GIC_GICD_ITARGETSR_PER_REG - 1) ) / 
		 GIC_GICD_ITARGETSR_PER_REG ) > i; ++i) 
		GICD_REGISTER->GIC_GICD_ITARGETSR[i] = 
			(u32)GIC_GICD_ITARGETSR_CORE0_TARGET_BMAP;

	/* Set trigger type for all peripheral interrupts level triggered */
	for (i = GIC_INTNO_PPI0 / GIC_GICD_ICFGR_PER_REG;
	     (GIC_INT_MAX + (GIC_GICD_ICFGR_PER_REG - 1)) / GIC_GICD_ICFGR_PER_REG > i; ++i)
		GICD_REGISTER->GIC_GICD_ICFGR[i] = GIC_GICD_ICFGR_LEVEL;

	/* Enable distributor */
	GICD_REGISTER->GIC_GICD_CTLR = GIC_GICD_CTLR_ENABLE;
}


void gicd_disable_irq(i32 irq) {
	GICD_REGISTER->GIC_GICD_ICENABLER[ (irq / GIC_GICD_ICENABLER_PER_REG) ] = 
		1U << ( irq % GIC_GICD_ICENABLER_PER_REG );
}


void gicd_enable_irq(i32 irq) {

	GICD_REGISTER->GIC_GICD_ISENABLER[(irq / GIC_GICD_ISENABLER_PER_REG) ]=
		1U << ( irq % GIC_GICD_ISENABLER_PER_REG );

}


void gicd_clear_pending(i32 irq) {

	GICD_REGISTER->GIC_GICD_ICPENDR[ (irq / GIC_GICD_ICPENDR_PER_REG) ] = 
		1U << ( irq % GIC_GICD_ICPENDR_PER_REG );
}


static i32 gicd_probe_pending(i32 irq) {
	i32 is_pending;

	is_pending = ( GICD_REGISTER->GIC_GICD_ISPENDR[(irq / GIC_GICD_ISPENDR_PER_REG) ] &
	    ( 1U << ( irq % GIC_GICD_ISPENDR_PER_REG ) ) );

	return ( is_pending != 0 );
}


static void gicd_set_target(i32 irq, u32 p){
	u32  shift;
	u32  reg;

	shift = (irq % GIC_GICD_ITARGETSR_PER_REG) * GIC_GICD_ITARGETSR_SIZE_PER_REG;

	reg = GICD_REGISTER->GIC_GICD_ITARGETSR[irq / GIC_GICD_ITARGETSR_PER_REG];
	reg &= ~( ((u32)(0xff)) << shift);
	reg |= (p << shift);
	GICD_REGISTER->GIC_GICD_ITARGETSR[irq / GIC_GICD_ITARGETSR_PER_REG] = reg;
}


static void gicd_set_priority(i32 irq, u32 prio){
	u32  shift;
	u32    reg;

	shift = (irq % GIC_GICD_IPRIORITY_PER_REG) * GIC_GICD_IPRIORITY_SIZE_PER_REG;
	reg = GICD_REGISTER->GIC_GICD_IPRIORITYR[irq / GIC_GICD_IPRIORITY_PER_REG];
	reg &= ~(((u32)(0xff)) << shift);
	reg |= (prio << shift);
	GICD_REGISTER->GIC_GICD_IPRIORITYR[irq / GIC_GICD_IPRIORITY_PER_REG] = reg;
}


static void gicd_config(i32 irq, u32 config)
{
	u32	shift; 
	u32	  reg;

	shift = (irq % GIC_GICD_ICFGR_PER_REG) * GIC_GICD_ICFGR_SIZE_PER_REG; /* GICD_ICFGR has 16 fields, each field has 2bits. */

	reg = GICD_REGISTER->GIC_GICD_ICFGR[ irq / GIC_GICD_ICFGR_PER_REG];

	reg &= ~( ( (u32)(0x03) ) << shift );  /* Clear the field */
	reg |= ( ( (u32)config ) << shift );  /* Set the value to the field correponding to irq */
	GICD_REGISTER->GIC_GICD_ICFGR[ irq / GIC_GICD_ICFGR_PER_REG] = reg;
}


void gic_eoi(i32 irq) {
	gicd_clear_pending(irq);
}


void gic_init(void)
{

	init_gicd();
	init_gicc();
	gicd_config(TIMER_IRQ, GIC_GICD_ICFGR_EDGE);
	gicd_set_priority(TIMER_IRQ, 0 << GIC_PRI_SHIFT );  /* Set priority */
	gicd_set_target(TIMER_IRQ, 0x1);  /* processor 0 */
	gicd_clear_pending(TIMER_IRQ);
	gicd_enable_irq(TIMER_IRQ);
}


i32 gic_fetch_irq(i32 *irqp)
{
	i32 rc;
	i32 i;
	for( i = 0; GIC_INT_MAX > i; ++i) {
		if ( gicd_probe_pending(i) ) {
			rc = IRQ_FOUND;
			*irqp = i;
			goto found;
		}
	}

	rc = IRQ_NOT_FOUND ;
found:
	return rc;
}

