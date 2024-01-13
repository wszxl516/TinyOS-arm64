#![no_std]
#![no_main]

extern crate std;

use std::{pr_notice, shutdown, reboot, sleep_ms, read_line};
use arrayvec::ArrayString;

#[no_mangle]
pub fn main() -> isize {
    run_loop();
    0
}
pub fn run_loop() {
    pr_notice!("User shell started !!!\n");
    let mut line =  ArrayString::new();
    loop {
        pr_notice!("#>>");
        line.clear();
        read_line::<64>(&mut line);
        if line.is_empty() {
            continue
        }
        match line.as_str() {
            "exit" => {
                pr_notice!("\nexit!\n");
                break
            }
            "shutdown" => {
                pr_notice!("\nshutdown!\n");
                shutdown()
            },
            "reboot" => {
                pr_notice!("\nreboot!\n");
                reboot()
            }
            "help" | _=> {
                pr_notice!("\ncommand: \n\texit shutdown reboot help.\n");
            }
        }
        sleep_ms(20);

    }

}
