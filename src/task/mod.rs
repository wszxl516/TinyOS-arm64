use crate::pr_notice;
use crate::task::scheduler::add_task;
use crate::task::task::Task;

pub mod context;
pub mod scheduler;
pub mod task;
mod mem;



pub fn init(){
    pr_notice!("Init Scheduler\n");
    scheduler::init();
    pr_notice!("Init first user task\n");
    add_task(Task::init());
    scheduler::yield_current();
}
