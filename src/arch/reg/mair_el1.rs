//https://developer.arm.com/documentation/ddi0601/2023-06/AArch64-Registers/MAIR-EL1--Memory-Attribute-Indirection-Register--EL1-
#![allow(non_snake_case)]
#![allow(dead_code)]

#[allow(non_upper_case_globals)]
pub mod MAIR_EL1 {
    use crate::def_reg_fn;

    //Gather(G)/non-Gather(nG)
    //Reorder(R)/non-Reorder(nR)
    //Early Write Acknowledgement (E/nE)
    pub mod device {
        //Device-nGnRnE
        pub const nGnRnE: usize = 0b00 << 2;
        //Device-nGnRE
        pub const nGnRE: usize = 0b01 << 2;
        //Device-nGRE
        pub const nGRE: usize = 0b10 << 2;
        //Device-GRE
        pub const GRE: usize = 0b11 << 2;
    }

    pub mod normal {
        pub mod inner {
            pub const WriteThroughTransient: usize = 0b0000;
            // + Read/Write
            pub const NonCacheable: usize = 0b0100;
            // No Read/Write
            pub const WriteBackTransient: usize = 0b0100;
            // + Read/Write
            pub const WriteThroughNonTransient: usize = 0b1000;
            pub const WriteBackNonTransient: usize = 0b1100;
            pub const ReadAllocate: usize = 0b10;
            pub const WriteAllocate: usize = 0b01;
        }

        pub mod outer {
            pub const WriteThroughTransient: usize = 0b0000 << 4;
            // + Read/Write
            pub const NonCacheable: usize = 0b0100 << 4;
            // No Read/Write
            pub const WriteBackTransient: usize = 0b0100 << 4;
            // + Read/Write
            pub const WriteThroughNonTransient: usize = 0b1000 << 4;
            pub const WriteBackNonTransient: usize = 0b1100 << 4;
            pub const ReadAllocate: usize = 0b10 << 4;
            pub const WriteAllocate: usize = 0b01 << 4;
        }
    }

    #[inline(always)]
    pub fn attr(index: usize, value: usize) -> usize {
        value << (index * 8)
    }

    def_reg_fn!(usize, MAIR_EL1);
}
