use alloc::boxed::Box;
use core::arch::asm;
use core::fmt;
use core::sync::atomic::{AtomicU32, Ordering};

use crate::mm::address::PAGE_SIZE;
use crate::mm::page::PageTable;
use crate::pr_info;
use crate::task::context::{Entry, switch_context, TaskContext};

pub type TaskFn = fn(usize) -> !;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct TaskId(u32);

static NEXT_PID: AtomicU32 = AtomicU32::new(1);

impl TaskId {
    const IDLE_TASK_ID: Self = Self(0);

    fn alloc() -> Self {
        Self(NEXT_PID.fetch_add(1, Ordering::AcqRel))
    }
    #[allow(dead_code)]
    pub const fn as_usize(&self) -> u32 {
        self.0
    }
}

impl From<u32> for TaskId {
    fn from(pid: u32) -> Self {
        Self(pid)
    }
}

#[repr(C)]
pub struct Task {
    name: &'static str,
    pub ctx: TaskContext,
    pub entry: Entry,
    pub k_stack: Stack<PAGE_SIZE>,
    pid: TaskId,
    pub page: PageTable,
}

impl Task {
    fn idle_task(_: usize) {
        loop {
            pr_info!("idle\n");
            unsafe { asm!("wfi") }
        }
    }
    pub fn new_kernel(name: &'static str, entry: TaskFn, arg: usize) -> Self {
        let stack = Stack::<PAGE_SIZE>::new();
        Task {
            name,
            ctx: TaskContext::new(stack.top()),
            entry: Entry::Kernel {
                pc: entry as usize,
                arg,
            },
            k_stack: stack,
            pid: TaskId::alloc(),
            page: PageTable::new(),
        }
    }
    pub fn idle() -> Self {
        let stack = Stack::<PAGE_SIZE>::new();
        Task {
            name: "idle",
            ctx: TaskContext::new(stack.top()),
            entry: Entry::Kernel {
                pc: Self::idle_task as usize,
                arg: 0,
            },
            k_stack: stack,
            pid: TaskId::IDLE_TASK_ID,
            page: PageTable::new(),
        }
    }
    pub fn switch_to(&mut self, next: &Task) {
        unsafe {
            switch_context(&mut self.ctx, &next.ctx);
        }
    }
    #[allow(dead_code)]
    pub fn pid(&self) -> TaskId {
        self.pid
    }

    pub fn as_ptr(&mut self) -> *mut Self {
        &mut *self
    }
}

impl fmt::Debug for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Task").field("pid", &self.pid).finish()
    }
}

pub struct Stack<const N: usize>(&'static mut [u8; N]);

impl<const N: usize> Stack<N> {
    pub fn new() -> Self {
        Self {
            0: Box::leak(Box::new([0u8; N])),
        }
    }
    pub fn top(&self) -> usize {
        self.0.as_ptr_range().end as usize
    }
}
