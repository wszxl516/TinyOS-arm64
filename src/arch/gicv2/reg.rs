use tock_registers::registers::ReadWrite;

//https://developer.arm.com/documentation/ihi0048/b/Programmers--Model/CPU-interface-register-descriptions
#[repr(C)]
pub struct GICC {
    /* CPU Interface Control Register 0x000*/
    pub ctlr: ReadWrite<u32>,
    /* Interrupt Priority Mask Register 0x004 */
    pub pmr: ReadWrite<u32>,
    /* Binary Point Register 0x008*/
    pub bpr: ReadWrite<u32>,
    /* Interrupt Acknowledge Register 0x00C */
    pub iar: ReadWrite<u32>,
    /* End of Interrupt Register 0x010*/
    pub eoir: ReadWrite<u32>,
    /* Running Priority Register 0x014*/
    pub rpr: ReadWrite<u32>,
    /* Highest Pending Interrupt Register 0x018*/
    pub hpir: ReadWrite<u32>,
    /* Aliased Binary Point Register 0x01C*/
    pub abpr: ReadWrite<u32>,
    _reserved: [u8; 0xdc],
    /* CPU Interface Identification Register 0x0FC*/
    pub iidr: ReadWrite<u32>,
}


#[repr(C)]
pub struct GICD {
    /* Distributor Control Register 0x000*/
    pub ctlr: ReadWrite<u32>,
    /* Interrupt Controller Type Register 0x004*/
    pub typer: ReadWrite<u32>,
    /* Distributor Implementer Identification Register 0x008*/
    pub iidr: ReadWrite<u32>,
    _reserved0: [u8; 0x74],
    /* Interrupt Group Registers 0x080*/
    pub igroupr: [ReadWrite<u32>; 0x20],
    /* Interrupt Set-Enable Registers 0x100*/
    pub isenabler: [ReadWrite<u32>; 0x20],
    /* Interrupt Clear-Enable Registers 0x180*/
    pub icenabler: [ReadWrite<u32>; 0x20],
    /* Interrupt Set-Pending Registers 0x200*/
    pub ispendr: [ReadWrite<u32>; 0x20],
    /* Interrupt Clear-Pending Registers 0x280*/
    pub icpendr: [ReadWrite<u32>; 0x20],
    /* Interrupt Set-Active Registers 0x300*/
    pub isactiver: [ReadWrite<u32>; 0x20],
    /* Interrupt Clear-Active Registers 0x380*/
    pub icactiver: [ReadWrite<u32>; 0x20],
    /* Interrupt Priority Registers 0x400*/
    pub ipriority: [ReadWrite<u32>; 0x100],
    /* Interrupt Processor Targets Registers 0x800*/
    pub itargetsr: [ReadWrite<u32>; 0x100],
    /* Interrupt Configuration Registers 0xc00*/
    pub icfgr: [ReadWrite<u32>; 0x40],
    /* Non-secure Access Control Registers 0xd00*/
    _reserved1: [u8; 0x1c0],
    /* Non-secure Access Control Registers 0xe00*/
    pub nscar: [ReadWrite<u32>; 0x100],
    /* Software Generated Interrupt Register 0xf00*/
    pub sgir: ReadWrite<u32>,
    _reserved2: [u8; 12],
    pub cpendsgir: [ReadWrite<u32>; 4],
    pub spendsgir: [ReadWrite<u32>; 4],
}
