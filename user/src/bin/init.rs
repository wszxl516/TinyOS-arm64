#![no_std]
#![no_main]

extern crate std;

use std::{pr_notice, pr_info, read_key, shutdown, sleep_ms};

#[no_mangle]
pub fn main() -> ! {
    pr_info!("Ctrl-q to shutdown\n");
    loop {
        let key = read_key!();
        match key as u8 {
            0x11 => {
                pr_notice!("shutdown\n");
                shutdown()
            },
            0 => {}
            _ => pr_notice!("{}\n", key),
        }
        sleep_ms(10);
    }
}
