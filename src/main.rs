#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(asm_const)]
#![feature(const_mut_refs)]
#![feature(ascii_char)]
#![feature(strict_provenance)]
#![feature(slice_first_last_chunk)]
#![feature(slice_pattern)]
#![feature(ptr_metadata)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(stmt_expr_attributes)]
#![feature(const_option)]
#![feature(stdsimd)]
#![feature(mem_copy_fn)]
extern crate alloc;

mod arch;
mod common;
mod config;
mod devices;
mod mm;
mod task;

#[no_mangle]
fn kernel_main() -> ! {
    arch::reg::DAIF::Irq.disable();
    mm::init();
    arch::init();
    devices::init();
    #[cfg(feature = "test")]
    test::test_abort();
    task::scheduler::init();
    task::scheduler::yield_current();
    arch::reg::DAIF::Irq.enable();
    loop {}
}


#[cfg(feature = "test")]
mod test {
    #[inline(never)]
    #[no_mangle]
    pub fn test_abort() {
        use core::arch::asm;
        test_stacktrace();
        unsafe { asm!("ldr x0, [{tmp}, 0]", tmp = in(reg) usize::MAX) };
    }

    #[inline(never)]
    #[no_mangle]
    fn test_stacktrace() {
        use core::arch::asm;
        unsafe { asm!("ldr x0, [{tmp}, 0]", tmp = in(reg) usize::MAX) };
    }
}

