#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]
#![feature(strict_provenance)]

use core::panic::PanicInfo;

use syscall::{sys_write, sys_read, sys_shutdown};

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

