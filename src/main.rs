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
    platform_info();
    task::scheduler::init();
    task::scheduler::yield_current();
    arch::reg::DAIF::Irq.enable();
    #[cfg(feature = "test")]
    test::test_abort();
    loop {}
}


fn platform_info() {
    unsafe {
        let dtb = fdt::Fdt::from_ptr(arch::BOOT_ARGS[0].into_vaddr().as_usize() as *const u8).unwrap();
        if let Some(platform) = dtb.find_node("/platform-bus") {
            pr_notice!("Model: {}, Platform: {}\n",dtb.root().model() ,platform.compatible().unwrap().first())
        }
    }
}

#[cfg(feature = "test")]
mod test {
    #[inline(never)]
    pub fn test_abort() {
        use core::arch::asm;
        test1();
        unsafe { asm!("ldr x0, [{tmp}, 0]", tmp = in(reg) usize::MAX) };
    }

    #[inline(never)]
    fn test1() {
        use core::arch::asm;
        unsafe { asm!("ldr x0, [{tmp}, 0]", tmp = in(reg) usize::MAX) };
    }
}

