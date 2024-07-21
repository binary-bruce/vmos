mod context;
mod manager;
mod switch;
mod tcb;

pub use context::TaskContext;
use lazy_static::lazy_static;
use manager::{TaskManager, TaskManagerInner};
use tcb::{TaskControllBlock, TaskStatus};

use crate::{
    config::MAX_APP_NUM,
    loader::{get_num_app, init_trap_cx},
    sync::UPSafeCell,
};

lazy_static! {
    /// Global variable: TASK_MANAGER
    pub static ref TASK_MANAGER: TaskManager = {
        let num_app = get_num_app();
        let init_tcb = TaskControllBlock {
            task_cx: TaskContext::zero_init(),
            task_status: tcb::TaskStatus::UnInit,
        };
        let mut tasks = [init_tcb; MAX_APP_NUM];

        for (i, task) in tasks.iter_mut().enumerate() {
            let kernel_sp = init_trap_cx(i);
            task.task_cx = TaskContext::goto_restore(kernel_sp);
            task.task_status = TaskStatus::Ready;
        }

        TaskManager {
            num_app,
            inner: unsafe {
                UPSafeCell::new(TaskManagerInner {
                    tasks,
                    current_task: 0,
                })
            },
        }
    };
}

/// run first task
pub fn run_first_task() {
    TASK_MANAGER.run_first_task();
}

/// rust next task
fn run_next_task() {
    TASK_MANAGER.run_next_task();
}

/// suspend current task
fn mark_current_suspended() {
    TASK_MANAGER.mark_current_suspended();
}

/// exit current task
fn mark_current_exited() {
    TASK_MANAGER.mark_current_exited();
}

/// suspend current task, then run next task
pub fn suspend_current_and_run_next() {
    mark_current_suspended();
    run_next_task();
}

/// exit current task,  then run next task
pub fn exit_current_and_run_next() {
    mark_current_exited();
    run_next_task();
}
