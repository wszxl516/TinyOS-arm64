use crate::{pr_err, pr_info};
use crate::arch::trap::context::Context;

const SYSCALL_WRITE: usize = 64;

pub fn syscall(tf: &Context) -> isize {
    let syscall_id = tf.reg[8];
    match syscall_id {
        SYSCALL_WRITE => pr_info!("syscall test!\n"),
        _ => {
            pr_err!("Unsupported syscall_id: {}", syscall_id);
        }
    };
    0
}