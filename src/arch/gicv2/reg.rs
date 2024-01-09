use tock_registers::register_structs;
use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};

//https://developer.arm.com/documentation/ihi0048/b/Programmers--Model/CPU-interface-register-descriptions
//https://developer.arm.com/documentation/ddi0471/b/programmers-model/distributor-register-summary
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

register_structs! {
    #[repr(C)]
    pub GICD {
        /* Distributor Control Register 0x000*/
        (0x000 => pub ctlr: ReadWrite<u32>),
        /* Interrupt Controller Type Register 0x004*/
        (0x004 => pub typer: ReadWrite<u32>),
        /* Distributor Implementer Identification Register 0x008*/
        (0x008 => pub iidr: ReadWrite<u32>),
        //0xc
        (0x00c => _reserved0: [u8; 0x74]),
        /* Interrupt Group Registers 0x080*/
        (0x080 => pub igroupr: [ReadWrite<u32>; 0x20]),
        /* Interrupt Set-Enable Registers 0x100*/
        //SGIs and PPIs isenabler[0]
        //SPIs isenabler[1..]
        (0x100 => pub isenabler: [ReadWrite<u32>; 0x20]),
        //SGIs and PPIs icenabler[0]
        //SPIs icenabler[1..]
        /* Interrupt Clear-Enable Registers 0x180*/
        (0x180 => pub icenabler: [ReadWrite<u32>; 0x20]),
        /* Interrupt Set-Pending Registers 0x200*/
        (0x200 => pub ispendr: [ReadWrite<u32>; 0x20]),
        /* Interrupt Clear-Pending Registers 0x280*/
        (0x280 => pub icpendr: [ReadWrite<u32>; 0x20]),
        /* Interrupt Set-Active Registers 0x300*/
        (0x300 => pub isactiver: [ReadWrite<u32>; 0x20]),
        /* Interrupt Clear-Active Registers 0x380*/
        (0x380 => pub icactiver: [ReadWrite<u32>; 0x20]),
        /* Interrupt Priority Registers 0x400*/
        (0x400 => pub ipriority: [ReadWrite<u32>; 0x100]),
        /* Interrupt Processor Targets Registers 0x800*/
        (0x800 => pub itargetsr: [ReadWrite<u32>; 0x100]),
        /* Interrupt Configuration Registers 0xc00*/
        //0xC00 RO SGIs
        //
        //0xC04 RO PPIs
        //
        //0xC08-0xC7C RW SPIs
        (0xc00 => pub icfgr: [ReadWrite<u32>; 0x40]),
        /* Private Peripheral Interrupt Status Register 0xd00*/
        (0xd00 => pub ppisr: ReadOnly<u32>),
        /*Shared Peripheral Interrupt Status Registers*/
        (0xd04 => pub spisr: [ReadOnly<u32>; 0xe]),
        (0xd3c => __reserved_0: [ReadOnly<u32>; 0x71]),
        /* Software Generated Interrupt Register 0xf00*/
        (0xf00 => pub sgir: WriteOnly<u64>),
        (0xf08 =>  __reserved_1: WriteOnly<u64>),
        /*SGI Clear-Pending Registers*/
        (0xf10 => pub sgiclear: ReadWrite<u64>),
        (0xf18 => __reserved_2: [u32; 0x2]),
        /*SGI Set-Pending Registers*/
        (0xf20 => pub sgiset: ReadWrite<u64>),
        (0xf28 => __reserved_3: [u32; 0x2a]),
        /*Peripheral ID 4 Register*/
        (0xFD0 => pub pidr4: [ReadWrite<u32>; 0x4]),
        (0xFE0 => pub pidr0: [ReadWrite<u32>; 0x4]),
        /*Component ID 0 Register*/
        (0xFF0 => pub cidr: [ReadWrite<u32>; 0x4]),
        (0x1000 => @END),
    }
}
