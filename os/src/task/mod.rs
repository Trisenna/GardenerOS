mod context;
mod switch;
mod task;
mod manager;
mod processor;
mod pid;

use crate::loader::get_app_data_by_name;
use alloc::sync::Arc;
use lazy_static::*;
pub use context::TaskContext;
pub use task::{TaskControlBlock, TaskControlBlockInner, TaskStatus};
use crate::sync::UPSafeCell;
pub use manager::{add_task, fetch_task, TaskManager};
pub use processor::{
    run_tasks, current_task, current_user_token, current_trap_cx,
    schedule, take_current_task, suspend_current_and_run_next,
    exit_current_and_run_next,
};
pub use pid::{PidHandle, pid_alloc, KernelStack, kernel_stack_position};
pub use switch::__switch;

lazy_static! {
    pub static ref INITPROC: Arc<TaskControlBlock> = Arc::new(
        TaskControlBlock::new(get_app_data_by_name("initproc").unwrap())
    );
}

pub fn add_initproc() {
    add_task(INITPROC.clone());
}