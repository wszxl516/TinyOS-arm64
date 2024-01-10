#![no_std]
#![no_main]

extern crate std;

use std::{println, read_key, shutdown};


#[no_mangle]
pub fn main() -> ! {
    loop {
        let key = read_key!();
        match key {
            0x11 => {
                println!("shutdown");
                shutdown()
            },
            0 => {}
            _ => println!("{:#x}", key),
        }

    }
}
