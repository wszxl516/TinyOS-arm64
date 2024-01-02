use core::arch::asm;

use crate::arch::reg::DAIF;
use crate::common::queue::Queue;
use crate::pr_notice;
use crate::task::task::{Task, TaskFn};

static mut SCHEDULER: Scheduler = Scheduler::new();

pub struct Scheduler {
    queue: Queue<Task>,
}


impl Scheduler {
    pub const fn new() -> Self {
        Self {
            queue: Queue::<Task>::new(),
        }
    }
    pub fn init(&mut self) {
        self.queue.push_front(Task::idle());
    }
    pub fn switch(&mut self, current: *mut Task) {
        match self.next() {
            Some(next) => {
                if next != current {
                    unsafe { (*current).switch_to(&*next) }
                }
            }
            _ => {}
        }
    }
    #[no_mangle]
    pub fn yield_current(&mut self) {
        match self.current() {
            None => {}
            Some(current) => {
                DAIF::Irq.enable();
                self.switch(current);
                DAIF::Irq.disable();
            }
        };
    }
    pub fn add_task(&mut self, name: &'static str, func: TaskFn, arg: usize) {
        self.queue.push_back(Task::new_kernel(name, func, arg));
    }

    #[inline(always)]
    pub fn current(&mut self) -> Option<*mut Task> {
        match self.queue.head() {
            None => None,
            Some(current) => Some(current.as_ptr()),
        }
    }
    #[inline(always)]
    pub fn next(&mut self) -> Option<*mut Task> {
        match self.queue.next() {
            None => None,
            Some(next) => Some(next.as_ptr()),
        }
    }
}

#[inline(always)]
pub fn init() {
    unsafe { SCHEDULER.init() }
    add_task("task1: {}", |_| loop {
        for i in 0..10 {
            pr_notice!("Task1: {}\n", i);
            unsafe {
                asm!("wfi");
            }
        }
    }, 0);
    add_task("task2: {}", |_| loop {
        for i in 0..10 {
            pr_notice!("Task2: {}\n", i);
            unsafe {
                asm!("wfi");
            }
        }
    }, 0);
}

#[inline(always)]
pub fn yield_current() {
    unsafe { SCHEDULER.yield_current() }
}

#[inline(always)]
pub fn add_task(name: &'static str, func: TaskFn, arg: usize) {
    unsafe { SCHEDULER.add_task(name, func, arg) }
}

#[inline(always)]
pub fn current() -> Option<*mut Task> {
    unsafe { SCHEDULER.current() }
}
