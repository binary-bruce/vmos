use crate::{config::MAX_APP_NUM, sbi::shutdown, sync::UPSafeCell};

use super::{
    switch::__switch,
    tcb::{TaskControllBlock, TaskStatus},
    TaskContext,
};

pub struct TaskManager {
    pub(crate) num_app: usize,

    /// use inner value to get mutable access
    pub(crate) inner: UPSafeCell<TaskManagerInner>,
}

pub struct TaskManagerInner {
    /// task list
    pub(crate) tasks: [TaskControllBlock; MAX_APP_NUM],

    /// id of the current `Running` task
    pub(crate) current_task: usize,
}

impl TaskManager {
    pub(crate) fn run_first_task(&self) -> ! {
        println!("[task manager] Running first task");

        let mut inner = self.inner.exclusive_access();
        let task0 = &mut inner.tasks[0];
        task0.task_status = TaskStatus::Running;
        let first_task_cx = &task0.task_cx as *const TaskContext;
        drop(inner);

        let mut _unused = TaskContext::zero_init();
        unsafe {
            println!(
                "[task manager] Switching context for the first task: {:?}",
                *first_task_cx
            );
            __switch(&mut _unused as *mut TaskContext, first_task_cx);
        }

        panic!("unreachable in run_first_task!");
    }

    pub(crate) fn mark_current_suspended(&self) {
        self.set_currenct_status(TaskStatus::Ready);
    }

    pub(crate) fn mark_current_exited(&self) {
        self.set_currenct_status(TaskStatus::Exited);
    }

    fn set_currenct_status(&self, status: TaskStatus) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = status;
    }

    pub(crate) fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        (current + 1..current + self.num_app + 1)
            .map(|id| id % self.num_app)
            .find(|id| inner.tasks[*id].task_status == TaskStatus::Ready)
    }

    pub(crate) fn run_next_task(&self) {
        if let Some(next) = self.find_next_task() {
            let mut inner = self.inner.exclusive_access();
            let current = inner.current_task;
            inner.tasks[next].task_status = TaskStatus::Running;
            inner.current_task = next;

            let current_cx = &mut inner.tasks[current].task_cx as *mut TaskContext;
            let next_cx = &mut inner.tasks[next].task_cx as *mut TaskContext;
            drop(inner);
            // before this, we should drop local variables that must be dropped manually
            unsafe {
                __switch(current_cx, next_cx);
            }
        } else {
            println!("All applications are completed!");
            shutdown(false);
        }
    }
}
