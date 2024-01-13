use lazy_static::lazy_static;

use crate::arch::reg::{DAIF, set_thread_pointer};
use crate::common::sync::Mutex;
use crate::mm::enable_table;
use crate::task::context::{switch_context, TaskContext};
use crate::task::queue::TaskQueue;
use super::{task::Task, types::TaskState};

lazy_static! {
    pub static ref SCHEDULER: Mutex<Scheduler> = {
        let mut s = Scheduler::new();
        s.init();
        Mutex::new(s)
    };
}

#[derive(PartialEq)]
pub enum State {
    Initialized,
    Running,
    Stopped,
}

impl State {
    pub fn is_running(&self) -> bool {
        self == &Self::Running
    }
}

pub struct Scheduler {
    queue: TaskQueue<Task>,
    idle: Option<Task>,
    state: State,
    current: Option<&'static mut Task>,
}

impl Scheduler {
    pub const fn new() -> Self {
        Self {
            queue: TaskQueue::<Task>::new(),
            idle: None,
            state: State::Stopped,
            current: None,
        }
    }
    pub fn init(&mut self) {
        self.idle.replace(Task::idle());
        self.state = State::Initialized;
    }
    unsafe fn switch(&mut self, current: *mut Task) {
        //start first task
        if !self.state.is_running() {
            self.state = State::Running;
                (*current).state = TaskState::Running;
                set_thread_pointer(current.addr());
                enable_table((*current).ctx.ttbr0_el1, false);
                switch_context(0 as *mut TaskContext, &mut (*current).ctx)
        }
        //switch task
        else {
            match self.next() {
                Some(next) => {
                        self.current.replace(&mut *next);
                    if next != current {
                            (*next).set_running();
                            if !(*current).state.is_exited() {
                                (*current).set_ready()
                            }
                            set_thread_pointer(next.addr());
                            enable_table((*next).ctx.ttbr0_el1, false);
                            switch_context(&mut (*current).ctx, &(*next).ctx)

                    }
                }
                None => {}
            }
        }
    }
    #[no_mangle]
    pub fn yield_current(&mut self) {
        match self.current() {
            //Scheduler not Initialized
            None => {}
            Some(current) => {
                DAIF::Irq.enable();
                unsafe { self.switch(current); }
                DAIF::Irq.disable();
            }
        };
    }

    pub fn exit_current(&mut self, exit_code: isize) -> ! {
        match self.current() {
            None => {}
            Some(current) => unsafe {
                (*current).set_exited();
                (*current).exit(exit_code)
            },
        }
        self.yield_current();
        unreachable!();
    }
    pub fn add_task(&mut self, task: Task) {
        self.queue.push_front(task);
    }
    pub fn idle(&mut self) -> Option<*mut Task> {
        match &mut self.idle {
            None => None,
            Some(idle) => Some(idle),
        }
    }
    pub fn current(&mut self) -> Option<*mut Task> {
        match  &mut self.current {
            None => self.idle(),
            Some(current) => Some(current.as_ptr()),
        }
    }
    pub fn next(&mut self) -> Option<*mut Task> {
        for _ in 0..self.queue.len {
            match self.queue.next() {
                None => return self.idle(),
                Some(next) => {
                    if next.state.is_ready() {
                        return Some(next.as_ptr());
                    }
                }
            }
        }
        self.idle()
    }
}

#[inline(always)]
pub fn yield_current() {
    let mut s = SCHEDULER.lock();
    unsafe {
        SCHEDULER.force_unlock();
    }
    s.get_mut().yield_current();
}

#[inline(always)]
pub fn add_task(task: Task) {
    match &mut SCHEDULER.lock() {
        lock => lock.add_task(task),
    }
}

#[inline(always)]
pub fn current() -> Option<*mut Task> {
    match &mut SCHEDULER.lock() {
        lock => lock.current(),
    }
}

#[inline(always)]
pub fn exit_current(code: isize) -> ! {
    let mut s = SCHEDULER.lock();
    unsafe {
        SCHEDULER.force_unlock();
    }
    s.get_mut().exit_current(code);
}
