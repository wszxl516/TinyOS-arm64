use alloc::boxed::Box;
use core::fmt;
use core::sync::atomic::{AtomicU32, Ordering};

use crate::arch::reg::wfi;
use crate::arch::trap::context::Context;
use crate::mm::{PAGE_SIZE};
use crate::{pr_info};
use crate::mm::flush::{dsb_all, isb_all};
use crate::task::context::{Entry, task_entry, TaskContext};
use crate::task::mem::UserSpace;

pub type TaskFn = fn(usize) -> !;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct TaskId(u32);
#[link_section = ".rodata"]
static BIN_INIT: &[u8] = include_bytes!(env!("INIT_BIN"));

impl TaskId {
    const IDLE_TASK_ID: Self = Self(0);

    pub(crate) fn alloc() -> Self {
        static NEXT_PID: AtomicU32 = AtomicU32::new(1);
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
    pub name: &'static str,
    pub ctx: TaskContext,
    pub entry: Entry,
    pub k_stack: Stack<PAGE_SIZE>,
    pub pid: TaskId,
    pub page: UserSpace,
}

impl Task {
    fn idle_task(_: usize) ->!{
        loop {
            pr_info!("idle\n");
            wfi()
        }
    }
    pub fn new_kernel(name: &'static str, entry: TaskFn, arg: usize, id: TaskId) -> Self {
        let stack = Stack::<PAGE_SIZE>::new();
        Task {
            name,
            ctx: TaskContext::new(stack.top()),
            entry: Entry::Kernel {
                pc: entry as usize,
                arg,
            },
            k_stack: stack,
            pid: id,
            page: UserSpace::empty(),
        }
    }
    pub fn idle() -> Self {
        Self::new_kernel("idle", Self::idle_task, 0, TaskId::IDLE_TASK_ID)
    }
    pub fn new_user(name: &'static str, data: &[u8]) -> Self {
        let mut vm = UserSpace::new();
        let (entry, stack_top) = vm.load_bin(data);
        let ttbr0_el1 = vm.root_phy_addr();
        let mut t = Task{
            name,
            ctx: TaskContext::default(),
            entry: Entry::User(Box::new(Context::new_user(entry, stack_top))),
            k_stack: Stack::new(),
            pid: TaskId::alloc(),
            page: vm,
        };
        t.ctx.init(task_entry as usize, t.k_stack.top(), ttbr0_el1);
        isb_all();
        dsb_all();
        t
    }
    #[inline(always)]
    pub fn init() -> Self {
        Self::new_user("init", BIN_INIT)
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
