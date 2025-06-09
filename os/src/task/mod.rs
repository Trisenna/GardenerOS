mod context;
mod switch;
mod task;
mod manager;
mod processor;
mod pid;

use crate::loader::{get_app_data, get_num_app};
use crate::mm::{MemorySet, KERNEL_SPACE};
use crate::sync::UPSafeCell;
use crate::trap::TrapContext;
use alloc::sync::Arc;
use lazy_static::*;
use manager::fetch_task;
use switch::__switch;
pub use context::TaskContext;
pub use manager::{add_task};
pub use pid::{PidHandle, pid_alloc, KernelStack};
pub use processor::{
    current_task,
    current_user_token,
    current_trap_cx,
    run_tasks,
    schedule,
    take_current_task,
};
pub use task::{TaskControlBlock, TaskStatus};

lazy_static! {
    pub static ref INITPROC: Arc<TaskControlBlock> = Arc::new(TaskControlBlock::new(
        get_app_data(0)
    ));
}

pub fn add_initproc() {
    add_task(INITPROC.clone());
}

pub fn suspend_current_and_run_next() {
    // There must be an application running.
    let task = take_current_task().unwrap();

    // ---- access current TCB exclusively
    let mut task_inner = task.inner_exclusive_access();
    let task_cx_ptr = &mut task_inner.task_cx as *mut TaskContext;
    // Change status to Ready
    task_inner.task_status = TaskStatus::Ready;
    drop(task_inner);
    // ---- release current PCB

    // push back to ready queue.
    add_task(task);
    // jump to scheduling cycle
    schedule(task_cx_ptr);
}

pub fn exit_current_and_run_next(exit_code: i32) {
    // take from Processor
    let task = take_current_task().unwrap();
    // **** access current TCB exclusively
    let mut inner = task.inner_exclusive_access();
    // Change status to Zombie
    inner.task_status = TaskStatus::Zombie;
    // Record exit code
    inner.exit_code = exit_code;
    // do not move to its parent but under initproc
    // ++++++ access initproc TCB exclusively
    {
        let mut initproc_inner = INITPROC.inner_exclusive_access();
        for child in inner.children.iter() {
            child.inner_exclusive_access().parent = Some(Arc::downgrade(&INITPROC));
            initproc_inner.children.push(child.clone());
        }
    }
    // ++++++ release parent PCB
    inner.children.clear();
    // deallocate user space
    inner.memory_set.recycle_data_pages();
    drop(inner);
    // **** release current PCB
    // drop task manually to maintain rc correctly
    drop(task);
    // we do not have to save task context
    let mut unused = TaskContext::zero_init();
    schedule(&mut unused as *mut _);
}

pub fn run_first_task() {
    let _task = take_current_task().unwrap();
    let task_cx_ptr = {
        let inner = _task.inner_exclusive_access();
        &inner.task_cx as *const TaskContext
    }; // inner guard 在这里被 drop，释放了对 _task 的借用
    let mut unused = TaskContext::zero_init();
    // 让 _task 在函数结束时自然释放，确保指针在使用期间数据结构保持有效
    unsafe {
        __switch(
            &mut unused as *mut TaskContext,
            task_cx_ptr,
        );
    }
    panic!("unreachable in run_first_task!");
}