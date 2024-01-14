use alloc::string::{String, ToString};
use core::fmt;
use core::fmt::{Display, Formatter};

use crate::arch::reg::wfi;
use crate::mm::{PAGE_SIZE, PhyAddr};
use crate::mm::flush::{dsb_all, isb_all};
use crate::task::context::{TaskContext, TaskEntry};
use crate::task::mem::UserSpace;
use crate::task::scheduler;
use super::types::{KernelStack, TaskId, TaskState};

pub const KERNEL_STACK_SIZE: usize= PAGE_SIZE * 4;
pub type TaskFn = fn(usize) -> isize;


#[link_section = ".rodata"]
static BIN_INIT: &[u8] = include_bytes!(env!("INIT_BIN"));


#[repr(C)]
#[derive(Clone)]
pub struct Task {
    pub name: String,
    pub state: TaskState,
    pub ctx: TaskContext,
    pub exit_code: isize,
    pub entry: TaskEntry,
    pub k_stack: KernelStack<KERNEL_STACK_SIZE>,
    pub pid: TaskId,
    pub page: UserSpace,
}
impl Display for Task{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Task:  {{ name: {} pid: {} {} task }}",
               self.name,
               self.pid.as_usize(),
               match self.entry {
                   TaskEntry::Kernel { .. } => "kernel",
                   TaskEntry::User(_) => "user"
               })
    }
}

impl Task {
    fn idle_task(_: usize) -> isize{
        loop {
            scheduler::yield_current();
            wfi()
        }
    }
    pub fn new_kernel(name: String,entry: TaskFn, arg: usize, id: TaskId) -> Self {
        let stack = KernelStack::new();
        Task {
            name,
            state: TaskState::Ready,
            ctx: TaskContext::new(stack.top(), PhyAddr::new(0)),
            exit_code: 0,
            entry: TaskEntry::new_kernel(entry as usize, arg),
            k_stack: stack,
            pid: id,
            page: UserSpace::empty(),
        }
    }
    pub fn idle() -> Self {
        Self::new_kernel("idle".to_string(),Self::idle_task, 0, TaskId::IDLE_TASK_ID)
    }
    pub fn new_user(name: String, data: &[u8]) -> Self {
        let mut vm = UserSpace::new();
        let (entry, stack_top) = vm.load_bin(data);
        let page_table_root = vm.root_addr();
        let k_stack =  KernelStack::new();
        let t = Task{
            name,
            state: TaskState::Ready,
            ctx: TaskContext::new(k_stack.top(), page_table_root),
            exit_code: 0,
            entry: TaskEntry::new_user(entry, stack_top),
            k_stack,
            pid: TaskId::alloc(),
            page: vm,
        };
        isb_all();
        dsb_all();
        t
    }
    #[inline(always)]
    pub fn init() -> Self {
        Self::new_user("init".to_string() ,BIN_INIT)
    }

    #[allow(dead_code)]
    pub fn pid(&self) -> TaskId {
        self.pid
    }

    #[inline(always)]
    pub fn set_ready(&mut self){
        self.state = TaskState::Ready
    }
    #[inline(always)]
    pub fn set_running(&mut self){
        self.state = TaskState::Running
    }
    #[inline(always)]
    pub fn set_exited(&mut self){
        self.state = TaskState::Exited
    }
    pub fn exit(&mut self, code: isize){
        self.exit_code = code
    }

    pub fn as_ptr(&mut self) -> *mut Self {
        &mut *self
    }
}

impl fmt::Debug for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Task").field("pid", &self.pid).finish()
    }
}


