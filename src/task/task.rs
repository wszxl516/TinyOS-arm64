use alloc::boxed::Box;
use core::fmt;
use core::sync::atomic::{AtomicU32, Ordering};

use crate::arch::reg::wfi;
use crate::arch::trap::context::Context;
use crate::mm::{PAGE_SIZE, PageTable, PTEFlags, VirtAddr};
use crate::mm::heap::page_alloc;
use crate::{align_up, pr_info};
use crate::mm::flush::{dsb_all, isb_all};
use crate::task::context::{Entry, task_entry, TaskContext};
pub const USR_STACK_SIZE: usize = PAGE_SIZE * 4;
pub type TaskFn = fn(usize) -> !;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct TaskId(u32);
#[link_section = ".rodata"]
static BIN_INIT: &[u8] = include_bytes!("../../init.bin");

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
    pub page: PageTable,
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
            page: PageTable::empty(),
        }
    }
    pub fn idle() -> Self {
        Self::new_kernel("idle", Self::idle_task, 0, TaskId::IDLE_TASK_ID)
    }
    pub fn init() -> Self {
        let mut vm = PageTable::empty();
        vm.init();
        let bin_size = align_up!(BIN_INIT.len(), PAGE_SIZE);
        let text_addr = page_alloc(bin_size / PAGE_SIZE);
        unsafe {
            core::slice::from_raw_parts_mut(text_addr.as_mut_ptr(), BIN_INIT.len()).copy_from_slice(BIN_INIT)
        }

        let text_start = VirtAddr::new(VirtAddr::USER_START);
        vm.map_area(text_start, text_addr.as_phy(), bin_size, PTEFlags::RX | PTEFlags::U, true);
        let stack_addr = page_alloc(USR_STACK_SIZE / PAGE_SIZE);
        let stack_start = VirtAddr::new(VirtAddr::USER_STACK_START);
        vm.map_area(stack_start, stack_addr.as_phy(), USR_STACK_SIZE, PTEFlags::RW | PTEFlags::U, true);

        let (entry, stack_top) = (text_start.as_usize(), stack_start.as_usize() + USR_STACK_SIZE);

        let mut t = Task{
            name: "init",
            ctx: TaskContext::default(),
            entry: Entry::User(Box::new(Context::new_user(entry, stack_top))),
            k_stack: Stack::new(),
            pid: TaskId::alloc(),
            page: vm,
        };
        t.ctx.init(task_entry as usize, t.k_stack.top(), vm.root_phy_addr());
        isb_all();
        dsb_all();
        t
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
