#ifndef __GIC_H__
#define __GIC_H__ 
#include "config.h"
#include "mmu.h"
#include "stdtypes.h"

#define GIC_GICD_BASE		                    (PHY_2_VIR(GIC_BASE_ADDR))  /* GICD MMIO base address */
#define GIC_GICC_BASE		                    (PHY_2_VIR(GIC_BASE_ADDR + 0x10000)) /* GICC MMIO base address */

#define GIC_GICD_INT_PER_REG			        (32)	/* 32 interrupts per reg */
#define GIC_GICD_IPRIORITY_PER_REG		        (4)		/* 4 priority per reg */
#define GIC_GICD_IPRIORITY_SIZE_PER_REG	        (8) 	/* priority element size */
#define GIC_GICD_ITARGETSR_CORE0_TARGET_BMAP    (0x01010101) /* CPU interface 0 */
#define GIC_GICD_ITARGETSR_PER_REG		        (4) 
#define GIC_GICD_ITARGETSR_SIZE_PER_REG	        (8) 
#define GIC_GICD_ICFGR_PER_REG			        (16) 
#define GIC_GICD_ICFGR_SIZE_PER_REG		        (2) 
#define GIC_GICD_ICENABLER_PER_REG		        (32)
#define GIC_GICD_ISENABLER_PER_REG		        (32)
#define GIC_GICD_ICPENDR_PER_REG		        (32)
#define GIC_GICD_ISPENDR_PER_REG		        (32)
/* 8.13.7 GICC_CTLR, CPU Interface Control Register */
#define GICC_CTLR_ENABLE			            (0x1)	/* Enable GICC */
#define GICC_CTLR_DISABLE			            (0x0)	/* Disable GICC */

/* 8.13.14 GICC_PMR, CPU Interface Priority Mask Register */
#define GICC_PMR_PRIO_MIN			            (0xff)	/* The lowest level mask */
#define GICC_PMR_PRIO_HIGH			            (0x0)	/* The highest level mask */

/* 8.13.6 GICC_BPR, CPU Interface Binary Point Register */
/* In systems that support only one Security state, when GICC_CTLR.CBPR == 0, 
this register determines only Group 0 interrupt preemption. */
#define GICC_BPR_NO_GROUP			            (0x0)	/* handle all interrupts */

/* 8.13.11 GICC_IAR, CPU Interface Interrupt Acknowledge Register */
#define GICC_IAR_INTR_IDMASK		            (0x3ff)	/* 0-9 bits means Interrupt ID */
#define GICC_IAR_SPURIOUS_INTR		            (0x3ff)	/* 1023 means spurious interrupt */


/* 8.9.4 GICD_CTLR, Distributor Control Register */
#define GIC_GICD_CTLR_ENABLE	                (0x1)	/* Enable GICD */
#define GIC_GICD_CTLR_DISABLE	                (0x0)	/* Disable GICD */

/* 8.9.7 GICD_ICFGR<n>, Interrupt Configuration Registers */
#define GIC_GICD_ICFGR_LEVEL	                (0x0)	/* level-sensitive */
#define GIC_GICD_ICFGR_EDGE		                (0x2)	/* edge-triggered */

/*
 * GIC on QEMU Virt 
 */
#define QEMU_VIRT_GIC_INT_MAX		            (64)
#define QEMU_VIRT_GIC_PRIO_MAX		            (16)
/* SGI: Interrupt IDs 0-15 */
/* PPI: Interrupt IDs 16-31 */
/* SPI: Interrupt IDs 32-63 */
#define QEMU_VIRT_GIC_INTNO_SGIO	            (0)
#define QEMU_VIRT_GIC_INTNO_PPIO	            (16)
#define QEMU_VIRT_GIC_INTNO_SPIO	            (32)

#define GIC_INT_MAX					            (QEMU_VIRT_GIC_INT_MAX)
#define GIC_PRIO_MAX				            (QEMU_VIRT_GIC_PRIO_MAX)
#define GIC_INTNO_SGI0				            (QEMU_VIRT_GIC_INTNO_SGIO)
#define GIC_INTNO_PPI0				            (QEMU_VIRT_GIC_INTNO_PPIO)
#define GIC_INTNO_SPI0				            (QEMU_VIRT_GIC_INTNO_SPIO)

#define GIC_PRI_SHIFT				            (4)
#define GIC_PRI_MASK				(           0x0f)

#define TIMER_IRQ					            (27)  /** Timer IRQ  */
#define IRQ_FOUND                               (1)
#define IRQ_NOT_FOUND                           (0)

/* 8.12 The GIC CPU interface register map */
typedef REG struct{
    /* CPU Interface Control Register 0x000*/
    REG u32 GIC_GICC_CTLR;
    /* Interrupt Priority Mask Register 0x004 */
    REG u32 GIC_GICC_PMR;
    /* Binary Point Register 0x008*/
    REG u32 GIC_GICC_BPR;
    /* Interrupt Acknowledge Register 0x00C */
    REG u32 GIC_GICC_IAR;
    /* End of Interrupt Register 0x010*/
    REG u32 GIC_GICC_EOIR;
    /* Running Priority Register 0x014*/
    REG u32 GIC_GICC_RPR;
    /* Highest Pending Interrupt Register 0x018*/
    REG u32 GIC_GICC_HPIR;
    /* Aliased Binary Point Register 0x01C*/
    REG u32 GIC_GICC_ABPR;
    _Reserved _Reserved0[0xdc];
    /* CPU Interface Identification Register 0x0FC*/
    REG u32 GIC_GICC_IIDR;

} GICC_REG;



typedef REG struct{
    /* Distributor Control Register 0x000*/
    REG u32 GIC_GICD_CTLR;
    /* Interrupt Controller Type Register 0x004*/
    REG u32 GIC_GICD_TYPER;
    /* Distributor Implementer Identification Register 0x008*/
    REG u32 GIC_GICD_IIDR;
    _Reserved _Reserved0[0x74];
    /* Interrupt Group Registers 0x080*/
    REG u32 GIC_GICD_IGROUPR[0x20];
    /* Interrupt Set-Enable Registers 0x100*/
    REG u32 GIC_GICD_ISENABLER[0x20];
    /* Interrupt Clear-Enable Registers 0x180*/
    REG u32 GIC_GICD_ICENABLER[0x20];
    /* Interrupt Set-Pending Registers 0x200*/
    REG u32 GIC_GICD_ISPENDR[0x20];
    /* Interrupt Clear-Pending Registers 0x280*/
    REG u32 GIC_GICD_ICPENDR[0x20];
    /* Interrupt Set-Active Registers 0x300*/
    REG u32 GIC_GICD_ISACTIVER[0x20];
    /* Interrupt Clear-Active Registers 0x380*/
    REG u32 GIC_GICD_ICACTIVER[0x20];
    /* Interrupt Priority Registers 0x400*/
    REG u32 GIC_GICD_IPRIORITYR[0x100];
    /* Interrupt Processor Targets Registers 0x800*/
    REG u32 GIC_GICD_ITARGETSR[0x100];
    /* Interrupt Configuration Registers 0xc00*/
    REG u32 GIC_GICD_ICFGR[0x40];
    /* Non-secure Access Control Registers 0xd00*/
    _Reserved _Reserved1[0x1c0];
    /* Non-secure Access Control Registers 0xe00*/
    REG u32 GIC_GICD_NSCAR[0x100];
    /* Software Generated Interrupt Register 0xf00*/
    REG u32 GIC_GICD_SGIR;
    _Reserved _Reserved2[12];
    REG u32 GIC_GICD_CPENDSGIR[4];
    REG u32 GIC_GICD_SPENDSGIR[4];

}GICD_REG;





void gic_init(void);
void gic_eoi(int irq);
int gic_fetch_irq(int *irqp);
void gicd_disable_int(int irq);
void gicd_enable_int(int irq);
void gicd_clear_pending(int irq);
void enable_irq(void);
void disable_irq(void);
#endif  //__GIC_H__
