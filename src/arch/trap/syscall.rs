use alloc::string::String;
use crate::{pr_err, print};
use crate::arch::psci::psci_cpu_off;
use crate::arch::trap::context::Context;
use crate::devices::gets;
const SYSCALL_SHUTDOWN: usize = 0;
const SYSCALL_READ: usize = 63;
const SYSCALL_WRITE: usize = 64;

#[no_mangle]
pub fn syscall(context: &Context) -> isize {
    let syscall_id = context.reg[8];
    match syscall_id {
        SYSCALL_WRITE => {
            match context.reg[0] {
                1 => {
                    let s = unsafe { String::from_utf8_lossy(core::slice::from_raw_parts(context.reg[1] as *const u8, context.reg[2]))};
                    print!("{}", s);
                    return context.reg[2] as isize
                }
                _ => {}
            }
        },
        SYSCALL_READ => {
            match context.reg[0] {
                0 => {
                    let mut buffer = [0u8;1];
                    let ret = gets(&mut buffer);
                    unsafe { (context.reg[1] as *mut u8).write(buffer[0]) };
                    return ret as isize
                }
                _ => {}
            }
        },
        SYSCALL_SHUTDOWN => {
            psci_cpu_off()
        }
        _ => {
            pr_err!("Unsupported syscall_id: {}\n", syscall_id);
        }
    };
    0
}
