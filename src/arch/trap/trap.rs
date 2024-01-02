use core::arch::global_asm;

use crate::{get_bit, pr_err, println, reg_read_p};
use crate::arch::gicv2::{ack_irq, fetch_handler, fetch_irq};
use crate::arch::reg::DAIF;
use crate::arch::trap::syscall::syscall;

use super::context::Context;
use super::types::{SyncException, SyncExceptionType};

global_asm!(include_str!("../macros.S"), include_str!("trap.S"));

#[no_mangle]
fn invalid_exception(frame: &Context) {
    pr_err!("{}\n", frame);
    frame.stacktrace();
    panic!()
}

#[no_mangle]
fn sync_exception(frame: &Context) -> isize {
    DAIF::Irq.disable();

    let ec = SyncException::new();
    match ec.ec {
        SyncExceptionType::UnknownReason => {}
        SyncExceptionType::TrappedWFIorWFE => {}
        SyncExceptionType::TrappedSimdOrFloatingPoint => {
            return 0;
        }
        SyncExceptionType::IllegalExecutionState => {}
        SyncExceptionType::SVCAArch64 => {
            return syscall(&frame);
        }
        SyncExceptionType::TrappedMSROrMRSAArch64 => {}
        SyncExceptionType::ExceptionPointerAuthentication => {}
        SyncExceptionType::InstructionAbortLowLevel => {
            pr_err!("Instruction Abort LowLevel: PC at {:#018x}\n", frame.elr);
        }
        SyncExceptionType::InstructionAbortCurrentLevel => {
            pr_err!("instruction abort: PC at {:#018x}\n", frame.elr);
        }
        SyncExceptionType::PCAlignmentFault => {}
        SyncExceptionType::DataAbortLowLevel => {
            let far = reg_read_p!(far_el1);
            pr_err!(
                "{}: {} access LowLevel from PC {:#018x}, FAR {:#018x}, iss {:#018x}\n",
                ec,
                match get_bit!(ec.iss, 6) {
                    1 => "Write",
                    _ => "Read",
                },
                frame.elr,
                far,
                ec.iss,
            );
        }
        SyncExceptionType::DataAbortCurrentLevel => {
            let far = reg_read_p!(far_el1);
            pr_err!(
                "{}: {} access from PC {:#018x}, FAR {:#018x}, iss {:#018x}\n",
                ec,
                match get_bit!(ec.iss, 6) {
                    1 => "Write",
                    _ => "Read",
                },
                frame.elr,
                far,
                ec.iss,
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
    pr_err!("{}\n", frame);
    frame.stacktrace();
    DAIF::Irq.enable();
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
                    pr_err!("Unknown platform_irq: {:?}", irq)
                }
                Some(handler) => ret = handler(irq),
            }
            ack_irq(irq)
        }
    }
    ret
}

#[no_mangle]
fn thread_preempt() {
    println!("thread_preempt")
}

#[no_mangle]
fn platform_fiq() {
    println!("platform_fiq")
}
