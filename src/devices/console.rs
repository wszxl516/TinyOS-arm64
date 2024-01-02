use core::fmt;
use core::fmt::Write;

use arrayvec::ArrayVec;

use crate::arch::{setup_irq, TriggerMode};
use crate::devices::uart::{Pl011Uart, Read};
use crate::mm::address::VirtAddr;

use super::super::config::{PL011_IRQ, UART_ADDRESS};

static mut UART: Pl011Uart = Pl011Uart::new(UART_ADDRESS);
static mut UART_RX_BUFFER: ArrayVec<u8, 64> = ArrayVec::new_const();

pub fn puts(args: fmt::Arguments) {
    unsafe { UART.write_fmt(args).unwrap() }
}

#[macro_export]
macro_rules! gets {
    ($buffer: expr) => {
        crate::devices::console::gets($buffer)
    };
}
#[macro_export]
macro_rules! read_key {
    () => {{
        let mut key = [0u8; 1];
        while crate::devices::console::gets(&mut key) == 0 {}
        key[0]
    }};
}

#[allow(dead_code)]
pub fn gets(buffer: &mut [u8]) -> usize {
    let mut n = 0;
    unsafe {
        while !UART_RX_BUFFER.is_empty() && n < buffer.len() {
            match UART_RX_BUFFER.pop() {
                None => {}
                Some(c) => {
                    buffer[n] = c;
                    n += 1;
                }
            }
        }
        n
    }
}

fn pl011uart_irq_handler(_irq: u32) -> i32 {
    unsafe {
        match UART.is_rx_interrupt() {
            true => {
                if UART_RX_BUFFER.is_full() {
                    UART_RX_BUFFER.clear()
                }
                while !UART.rx_is_empty() {
                    match UART.read_char() {
                        None => {}
                        Some(c) => UART_RX_BUFFER.push(c),
                    }
                }
            }
            false => {}
        }
    }
    0
}

pub fn setup_console() {
    unsafe {
        UART.reset_base_address(VirtAddr::from_phy(UART_ADDRESS).as_usize());
        UART.init(0, 0)
    };
    setup_irq(PL011_IRQ, TriggerMode::Level, pl011uart_irq_handler);
}
