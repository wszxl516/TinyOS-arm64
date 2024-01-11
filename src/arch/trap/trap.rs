use core::arch::global_asm;

use crate::{get_bit, pr_err, println, reg_read_p};
use crate::arch::{ack_irq, fetch_handler, fetch_irq};
use crate::arch::trap::syscall::syscall;

use super::context::Context;
use super::types::{SyncException, SyncExceptionType};

global_asm!(include_str!("../macros.S"), include_str!("trap.S"));

#[no_mangle]
fn invalid_exception(context: &Context) {
    pr_err!("{}\n", context);
    context.stacktrace();
    panic!()
}

#[no_mangle]
fn sync_exception(context: &mut Context){
    let ec = SyncException::new();
    let far = reg_read_p!(far_el1);
    match ec.ec {
        SyncExceptionType::UnknownReason => {}
        SyncExceptionType::TrappedWFIorWFE => {
            pr_err!("{:?}\n", ec.ec);
        }
        SyncExceptionType::TrappedSimdOrFloatingPoint => {
        }
        SyncExceptionType::IllegalExecutionState => {}
        SyncExceptionType::SVCAArch64 => {
            syscall(context);
            return;
        }
        SyncExceptionType::TrappedMSROrMRSAArch64 => {}
        SyncExceptionType::ExceptionPointerAuthentication => {}
        SyncExceptionType::InstructionAbortLowLevel => {
            pr_err!("Instruction Abort LowLevel: PC at {:#018x} iss: {:#x} {}\n", context.elr, ec.iss,  ec.fault_msg());
            pr_err!("{}\n", context);
            return;

        }
        SyncExceptionType::InstructionAbortCurrentLevel => {
            pr_err!("instruction abort: PC at {:#018x} {:#x} {}\n", context.elr, ec.iss, ec.fault_msg());

        }
        SyncExceptionType::PCAlignmentFault => {}
        SyncExceptionType::DataAbortLowLevel => {
            pr_err!(
                "{}: {} access LowLevel from PC {:#018x}, FAR {:#018x}, iss {:#018x} {}\n",
                ec,
                match get_bit!(ec.iss, 6) {
                    1 => "Write",
                    _ => "Read",
                },
                context.elr,
                far,
                ec.iss,
                ec.fault_msg()
            );
            pr_err!("{}\n", context);
            return;
        }
        SyncExceptionType::DataAbortCurrentLevel => {
            pr_err!(
                "{}: {} access from PC {:#018x}, FAR {:#018x}, iss {:#018x} {}\n",
                ec,
                match get_bit!(ec.iss, 6) {
                    1 => "Write",
                    _ => "Read",
                },
                context.elr,
                far,
                ec.iss,
                ec.fault_msg()
            );
        }
        SyncExceptionType::SPAlignmentFault => {}
        SyncExceptionType::SErrorInterrupt => {}
        SyncExceptionType::BreakpointLowerException => {}
        SyncExceptionType::BreakpointCurrentLevel => {}
        SyncExceptionType::SoftwareStepLowLevel => {}
        SyncExceptionType::SoftwareStepCurrentLevel => {}
        SyncExceptionType::WatchpointLowLevel => {}
        SyncExceptionType::WatchpointCurrentLevel => {}
        SyncExceptionType::BRKInstructionAArch64 => {}
    }
    pr_err!("{}\n", context);
    context.stacktrace();
    panic!()
}

#[no_mangle]
fn platform_irq() -> i32 {
    let mut ret = 0;
    match fetch_irq() {
        None => {}
        Some(irq) => {
            match fetch_handler(irq) {
                None => {
                    panic!("Unknown platform_irq: {:?}", irq.0)
                }
                Some(handler) => ret = handler(irq),
            }
            ack_irq(irq)
        }
    }
    ret
}

#[no_mangle]
fn platform_fiq() {
    println!("platform_fiq")
}
