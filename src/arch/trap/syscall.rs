use alloc::string::String;
use alloc::vec;

use crate::{pr_err, print};
use crate::arch::psci::{psci_cpu_off, psci_cpu_rest};
use crate::devices::gets;
use crate::mm::{UserBuffer, UserPtr};
use crate::task::scheduler;

const SYSCALL_SHUTDOWN: usize = 142;
const SYSCALL_READ: usize = 63;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

#[no_mangle]
pub fn syscall(syscall_id: usize, args: [usize; 6]) -> usize {
    match syscall_id {
        SYSCALL_WRITE => sys_write(args[0], UserPtr::<u8>::new(args[1], args[2])),
        SYSCALL_READ => sys_read(args[0], &mut UserPtr::<u8>::new(args[1], args[2])),
        SYSCALL_SHUTDOWN =>{
            match args[0] {
                0 => psci_cpu_off(),
                _ => psci_cpu_rest()
            }
        },
        SYSCALL_EXIT => scheduler::exit_current(args[0] as isize),
        _ => {
            pr_err!("Unsupported syscall_id: {}\n", syscall_id);
            0
        }
    }
}

pub fn sys_write(fd: usize, ptr :UserPtr<u8>)-> usize{
    let ret = ptr.len();
    match fd {
        1 => {
            print!("{}",String::copy_from_user(ptr).unwrap());
        }
        _ => {}
    }
    ret
}


pub fn sys_read(fd: usize, ptr: &mut UserPtr<u8>)-> usize{
    let ret;
    match fd {
        0 => {
            let mut buffer = vec![0; ptr.len()];
            ret = gets(&mut buffer);
            buffer.copy_to_user(ptr);
        }
        _ => unreachable!()
    }
    ret
}
