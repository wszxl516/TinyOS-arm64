use crate::arch::reg::{wfi, DAIF};
use crate::common::queue::Queue;
use crate::pr_notice;
use crate::task::context::{switch_context, TaskContext};
use crate::task::task::{Task, TaskFn};

static mut SCHEDULER: Scheduler = Scheduler::new();


#[derive(PartialEq)]
pub enum State{
    Initialized,
    Running,
    Stopped,
}
impl State{
    pub fn is_running(&self) -> bool{
        self == &Self::Running
    }
}
pub struct Scheduler {
    queue: Queue<Task>,
    idle: Option<Task>,
    state: State,
}

impl Scheduler {
    pub const fn new() -> Self {
        Self {
            queue: Queue::<Task>::new(),
            idle: None,
            state: State::Stopped,
        }
    }
    pub fn init(&mut self) {
        self.idle.replace(Task::idle());
        self.state = State::Initialized;

    }
    pub fn switch(&mut self, current: *mut Task) {
        //start first task
        if !self.state.is_running() {
            self.state = State::Running;
            unsafe { switch_context(0 as *mut TaskContext, &mut (*current).ctx) }
        }
        //switch task
        else {
            match self.next() {
                Some(next) => {
                    if next != current {
                        unsafe { switch_context(&mut (*current).ctx, &(*next).ctx) }
                    }
                }
                None =>{}
            }
        }
    }
    #[no_mangle]
    pub fn yield_current(&mut self) {
        match self.current() {
            //Scheduler not Initialized
            None => {},
            Some(current) => {
                DAIF::Irq.enable();
                self.switch(current);
                DAIF::Irq.disable();
            }
        };
    }
    pub fn add_task(&mut self, name: &'static str, func: TaskFn, arg: usize) {
        self.queue.push_front(Task::new_kernel(name, func, arg));
    }
    pub fn idle(&mut self) -> Option<*mut Task> {
        match &mut self.idle{
            None => None,
            Some(idle) => Some(idle)
        }
    }
    #[inline(always)]
    pub fn current(&mut self) -> Option<*mut Task> {
        match self.queue.head() {
            None => self.idle(),
            Some(current) => Some(current.as_ptr()),
        }
    }
    #[inline(always)]
    pub fn next(&mut self) -> Option<*mut Task> {
        match self.queue.next() {
            None => self.idle(),
            Some(next) => {
                Some(next.as_ptr())
            }
        }
    }
}

#[inline(always)]
pub fn init() {
    unsafe { SCHEDULER.init() }
    add_task(
        "task1: {}",
        |_| loop {
            for i in 0..10 {
                pr_notice!("Task1: {}\n", i);
                wfi();
            }
        },
        0,
    );
    add_task(
        "task2: {}",
        |_| loop {
            for i in 0..10 {
                pr_notice!("Task2: {}\n", i);
                wfi()
            }
        },
        0,
    );
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
