#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]
#![feature(strict_provenance)]
#![feature(stdsimd)]

use core::panic::PanicInfo;

use syscall::{sys_write, sys_read, sys_shutdown};
pub const CLOCK_FREQ:u64 =  0x3b9aca0;
pub const MS_PEER_CYCLE: u64 = CLOCK_FREQ / 1000;
pub mod syscall;
#[macro_use]
pub mod stdio;

#[no_mangle]
#[link_section = ".text._start"]
pub extern "C" fn _start() -> ! {
    main()
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> ! {
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

#[panic_handler]
pub fn panic(info: &PanicInfo) -> !{
    println!("{:?}", info);
    loop {

    }
}

