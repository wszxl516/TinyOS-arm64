use crate::pr_notice;
use crate::task::scheduler::add_task;
use crate::task::task::Task;

pub mod context;
pub mod scheduler;
pub mod task;
mod mem;
pub mod queue;
mod types;

pub fn init(){
    pr_notice!("Init Scheduler\n");
    let init = Task::init();
    pr_notice!("Start first user task {}\n", init.name);
    add_task(init);
    scheduler::yield_current();
}
