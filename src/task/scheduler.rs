use crate::arch::reg::{DAIF};
use crate::common::queue::Queue;
use crate::mm::enable_table;
use crate::task::context::{switch_context, TaskContext};
use crate::task::task::{Task};

static mut SCHEDULER: Scheduler = Scheduler::new();


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

            unsafe {
                enable_table((*current).ctx.ttbr0_el1, false);
                switch_context(0 as *mut TaskContext, &mut (*current).ctx)
            }
        }
        //switch task
        else {
            match self.next() {
                Some(next) => {
                    if next != current {
                        unsafe {
                            enable_table((*next).ctx.ttbr0_el1, false);
                            switch_context(&mut (*current).ctx, &(*next).ctx)
                        }
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
                self.switch(current);
                DAIF::Irq.disable();
            }
        };
    }
    pub fn add_task(&mut self, task: Task) {
        self.queue.push_front(task);
    }
    pub fn idle(&mut self) -> Option<*mut Task> {
        match &mut self.idle {
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
    add_task(Task::init());

}

#[inline(always)]
pub fn yield_current() {
    unsafe { SCHEDULER.yield_current() }
}

#[inline(always)]
pub fn add_task(task: Task) {
    unsafe { SCHEDULER.add_task(task) }
}

#[inline(always)]
pub fn current() -> Option<*mut Task> {
    unsafe { SCHEDULER.current() }
}
