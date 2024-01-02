use core::panic::PanicInfo;

use crate::arch::psci::psci_cpu_off;
use crate::pr_err;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    match info.location() {
        None => {}
        Some(location) => {
            pr_err!(
                "panic at: {} {}:{} ",
                location.file(),
                location.line(),
                location.column()
            );
        }
    }
    match info.message() {
        None => {}
        Some(message) => {
            pr_err!("message: {}", message);
        }
    }
    psci_cpu_off()
}