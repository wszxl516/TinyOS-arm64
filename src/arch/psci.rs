#![allow(dead_code)]

use core::arch::global_asm;

global_asm!(include_str!("macros.S"),
    r#"
    func_def _psci_call
    hvc     #0
    ret
    func_end _psci_call
    "#
);
const CPU_ON: u32 = 0xc4000003;
const CPU_OFF: u32 = 0x84000008;
const CPU_RESET: u32 = 0x84000009;
const CPU_SUSPEND: u32 = 0xc4000001;
const PSCI_VERSION: u32 = 0x84000000;

#[inline]
fn psci_call(arg0: u32, arg1: u32, arg2: u32, arg3: u32) -> i32 {
    extern "C" {
        pub fn _psci_call(arg0: u32, arg1: u32, arg2: u32, arg3: u32) -> i32;
    }
    unsafe { _psci_call(arg0, arg1, arg2, arg3) }
}

pub fn psci_version() -> i32 {
    psci_call(PSCI_VERSION, 0, 0, 0)
}

pub fn psci_cpu_off() -> ! {
    psci_call(CPU_OFF, 0, 0, 0);
    loop {}
}

pub fn psci_cpu_on(id: u32, entry: usize) -> i32 {
    psci_call(CPU_ON, id, entry as u32, id)
}

pub fn psci_cpu_rest() -> i32 {
    psci_call(CPU_RESET, 0, 0, 0)
}

pub fn psci_cpu_suspend() -> i32 {
    psci_call(CPU_SUSPEND, 0, 0, 0)
}
