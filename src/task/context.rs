use alloc::boxed::Box;
use core::arch::asm;

use crate::arch::reg::DAIF;
use crate::arch::trap::context::Context;
use crate::mm::PhyAddr;
use crate::task::scheduler;
use crate::task::task::TaskFn;

#[no_mangle]
#[naked]
#[allow(named_asm_labels)]
pub unsafe extern "C" fn switch_context(_current: *mut TaskContext, _next: *const TaskContext) {
    asm!(
    "   cmp     x0, xzr
        b.eq    first_task
        stp     x29, x30, [x0, 12 * 8]
        stp     x27, x28, [x0, 10 * 8]
        stp     x25, x26, [x0, 8 * 8]
        stp     x23, x24, [x0, 6 * 8]
        stp     x21, x22, [x0, 4 * 8]
        stp     x19, x20, [x0, 2 * 8]
        mov     x19, sp
        mrs     x20, tpidr_el0
        stp     x19, x20, [x0]
first_task:
        ldp     x19, x20, [x1]
        mov     sp, x19
        msr     tpidr_el0, x20
        ldp     x19, x20, [x1, 2 * 8]
        ldp     x21, x22, [x1, 4 * 8]
        ldp     x23, x24, [x1, 6 * 8]
        ldp     x25, x26, [x1, 8 * 8]
        ldp     x27, x28, [x1, 10 * 8]
        ldp     x29, x30, [x1, 12 * 8]

        ret",
    options(noreturn),
    )
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct TaskContext {
    pub sp: usize,
    pub tpidr_el0: usize,
    pub r19: usize,
    pub r20: usize,
    pub r21: usize,
    pub r22: usize,
    pub r23: usize,
    pub r24: usize,
    pub r25: usize,
    pub r26: usize,
    pub r27: usize,
    pub r28: usize,
    pub r29: usize,
    pub lr: usize,
    pub ttbr0_el1: usize,
}

impl TaskContext {
    pub fn entry() -> ! {
        DAIF::Irq.enable();
        match scheduler::current() {
            None => { panic!("no current!") }
            Some(task) => {
                unsafe {
                    let entry = &(*task).entry;
                    match entry {
                        TaskEntry::Kernel { pc, arg } => {
                            let entry: TaskFn = core::mem::transmute(*pc);
                            let code = entry(*arg);
                            (*task).exit(code);
                            scheduler::exit_current(code)
                        }
                        TaskEntry::User(tf) => {
                            tf.exec((*task).k_stack.top());
                        }
                    }
                }
            }
        }
    }

    pub fn new(stack_top: usize, page_table_root: PhyAddr) -> Self {
        Self {
            lr: Self::entry as usize,
            sp: stack_top,
            tpidr_el0: 0,
            r19: 0,
            r20: 0,
            r21: 0,
            r22: 0,
            r23: 0,
            r24: 0,
            r25: 0,
            r26: 0,
            r27: 0,
            r28: 0,
            r29: 0,
            ttbr0_el1: page_table_root.as_usize(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TaskEntry {
    Kernel { pc: usize, arg: usize },
    User(Box<Context>),
}

impl TaskEntry {
    pub fn new_kernel(pc: usize, arg: usize)-> Self{
        TaskEntry::Kernel {pc, arg}
    }
    pub fn new_user(entry: usize, stack_top: usize) -> Self{
        TaskEntry::User(Box::new(Context::new_user(entry, stack_top)))
    }
}

