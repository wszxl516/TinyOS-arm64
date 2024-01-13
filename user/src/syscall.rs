use core::arch::asm;

const SYSCALL_SHUTDOWN: usize = 142;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_READ: usize = 63;
const SYSCALL_EXIT: usize = 93;

#[no_mangle]
fn syscall(id: usize, args: [usize; 6]) -> isize {
    let ret;
    unsafe {
        asm!(
            "svc #0",
            inlateout("x0") args[0] => ret,
            in("x1") args[1],
            in("x2") args[2],
            in("x3") args[3],
            in("x4") args[4],
            in("x5") args[5],
            in("x8") id,
        );
    }
    ret
}
#[macro_export]
macro_rules! syscall_args {
    () => {
        [0, 0, 0, 0, 0, 0]
    };
    ($a1:expr) => {
        [$a1, 0, 0, 0, 0, 0]
    };

    ($a1:expr, $a2:expr) => {
        [$a1, $a2, 0, 0, 0, 0]
    };

    ($a1:expr, $a2:expr, $a3:expr) => {
        [$a1, $a2, $a3, 0, 0, 0]
    };

    ($a1:expr, $a2:expr, $a3:expr, $a4:expr) => {
        [$a1, $a2, $a3, $a4, 0, 0]
    };

    ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr) => {
        [$a1, $a2, $a3, $a4, $a5, 0]
    };

    ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        [$a1, $a2, $a3, $a4, $a5, $a6]
    };
}
#[inline(always)]
pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(
        SYSCALL_WRITE,
        syscall_args![fd, buffer.as_ptr().addr(), buffer.len()],
    )
}
#[inline(always)]
pub fn sys_read(fd: usize, buffer: &mut [u8]) -> isize {
    syscall(
        SYSCALL_READ,
        syscall_args![fd, buffer.as_mut_ptr().addr(), buffer.len()],
    )
}

pub fn sys_exit(exit_code: isize) -> ! {
    syscall(SYSCALL_EXIT, syscall_args![exit_code as usize]);
    panic!();
}

#[inline(always)]
pub fn sys_shutdown() -> isize {
    syscall(
        SYSCALL_SHUTDOWN,
        syscall_args![],
    )
}

#[inline(always)]
pub fn sys_reboot() -> isize {
    syscall(
        SYSCALL_SHUTDOWN,
        syscall_args![1],
    )
}
