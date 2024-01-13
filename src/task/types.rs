#![allow(dead_code)]
use alloc::boxed::Box;
use alloc::vec;
use core::sync::atomic::{AtomicU32, Ordering};

#[repr(transparent)]
pub struct KernelStack<const N: usize>(Box<[u8]>);

impl<const N: usize> KernelStack<N> {
    pub fn new() -> Self {
        Self(Box::from(vec![0u8; N]))
    }
    pub fn top(&self) -> usize {
        self.0.as_ptr_range().end.addr()
    }
    pub fn bottom(&self) -> usize{
        self.0.as_ptr_range().start.addr()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct TaskId(u32);
impl TaskId {
    pub const IDLE_TASK_ID: Self = Self(0);
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
#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TaskState {
    Ready = 1,
    Running = 2,
    Exited = 3,
}
impl TaskState{
    #[inline]
    pub const fn is_ready(&self) -> bool{
        match self {
            TaskState::Ready => true,
            _ => false
        }
    }
    #[inline]
    pub const fn is_running(&self) -> bool{
        match self {
            TaskState::Running => true,
            _ => false
        }
    }
    #[inline]
    pub const fn is_exited(&self) -> bool{
        match self {
            TaskState::Exited => true,
            _ => false
        }
    }
}