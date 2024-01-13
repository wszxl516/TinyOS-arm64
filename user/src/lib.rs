#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]
#![feature(strict_provenance)]
#![feature(stdsimd)]

use core::panic::PanicInfo;
use arrayvec::ArrayString;

use syscall::{sys_exit, sys_read, sys_shutdown, sys_write, sys_reboot};

pub const CLOCK_FREQ:u64 =  0x3b9aca0;
pub const MS_PEER_CYCLE: u64 = CLOCK_FREQ / 1000;
pub mod syscall;
#[macro_use]
pub mod stdio;

#[no_mangle]
#[link_section = ".text._start"]
pub extern "C" fn _start() -> ! {
    sys_exit(main())
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> isize {
    panic!("Cannot find main!");
}

#[inline(always)]
pub fn wait_cycle(){
    unsafe { core::arch::aarch64::__nop() }
}
#[inline(always)]
pub fn sleep_ms(ms: u32){
    for _ in 0..MS_PEER_CYCLE * ms as u64{
        wait_cycle()
    }
}
pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}
pub fn read(fd: usize, buf: &mut [u8]) -> isize {
    sys_read(fd, buf)
}

pub fn shutdown() ->!{
    sys_shutdown();
    loop {}
}
pub fn reboot() ->!{
    sys_reboot();
    loop {}
}

pub fn read_line<const CAP: usize>(buffer: &mut ArrayString<CAP>){
    loop {
        let mut key: [u8; 1] = [0u8; 1];
        read(0, &mut key);
        let c = key[0];
        match c {
            b'\r' | b'\n' => {
                pr_notice!();
                return;
            }
            b'\x7f' | b'\x08' => {
                if buffer.len() > 0 {
                    pr_notice!("{}", b'\x08' as char);
                    pr_notice!(" ");
                    pr_notice!("{}", b'\x08' as char);
                    buffer.pop().unwrap();
                }
            }
            _ => {
                pr_notice!("{}", c as char);
                if c != 0 {
                    buffer.push( c as char);
                }

            }
        }
        sleep_ms(20);
    }
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> !{
    println!("{:?}", info);
    shutdown();
}

