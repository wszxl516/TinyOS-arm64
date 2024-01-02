#![allow(dead_code)]

#[macro_use]
pub mod lds {
    #[macro_export]
    macro_rules! lds_address {
        ($value: ident) => {
            ::paste::paste! {
                unsafe { &crate::config::lds::[<__ $value>] as *const u8 as usize }
            }
        };
    }
    extern "C" {
        pub static __text_start: u8;
        pub static __text_end: u8;
        pub static __stack_start: u8;
        pub static __stack_end: u8;
        pub static __ro_start: u8;
        pub static __ro_end: u8;
        pub static __data_start: u8;
        pub static __data_end: u8;
        pub static __bss_start: u8;
        pub static __bss_end: u8;
        pub static __symbols_start: u8;
        pub static __symbols_end: u8;
        pub static __heap_start: u8;
    }
}

pub const UART_ADDRESS: usize = 0x9000000;
pub const MEM_SIZE: usize = 0x8000000;
pub const PL011_IRQ: u32 = 33;
pub const TIMER_IRQ: u32 = 30;
pub const GICD_BASE: usize = 0x8000000;
pub const GICC_BASE: usize = 0x8010000;

pub const GIC_SIZE: usize = 0x10000;

