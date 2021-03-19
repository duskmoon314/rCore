mod context;
mod switch;
mod task;

use crate::config::{APP_SIZE_LIMIT, MAX_APP_NUM, MAX_DURATION, USER_STACK_SIZE};
use crate::{
    loader::{get_num_app, init_app_cx},
    timer::get_time_ms,
};
use core::cell::RefCell;
use lazy_static::*;
use riscv::register::{sepc, sscratch};
use switch::__switch;
use task::{Stride, TaskControlBlock, TaskStatus};

pub use context::TaskContext;

pub struct TaskManager {
    num_app: usize,
    inner: RefCell<TaskManagerInner>,
}

struct TaskManagerInner {
    tasks: [TaskControlBlock; MAX_APP_NUM],
    current_task: usize,
    current_start: usize,
}

unsafe impl Sync for TaskManager {}

lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = {
        let num_app = get_num_app();
        let mut tasks = [TaskControlBlock {
            task_cx_ptr: 0,
            task_status: TaskStatus::UnInit,
            stride: Stride(0),
            priority: 16,
            duration: 0,
        }; MAX_APP_NUM];
        for (i, task) in tasks.iter_mut().enumerate() {
            task.task_cx_ptr = init_app_cx(i) as *const _ as usize;
            task.task_status = TaskStatus::Ready;
        }
        TaskManager {
            num_app,
            inner: RefCell::new(TaskManagerInner {
                tasks,
                current_task: 0,
                current_start: 0,
            }),
        }
    };
}

impl TaskManager {
    fn run_first_task(&self) {
        let mut inner = self.inner.borrow_mut();
        inner.tasks[0].task_status = TaskStatus::Running;
        inner.current_start = get_time_ms();
        let priority = inner.tasks[0].priority;
        inner.tasks[0].stride.update(priority);
        core::mem::drop(inner);
        let next_task_cx_ptr2 = self.inner.borrow().tasks[0].get_task_cx_ptr2();
        let _unused: usize = 0;
        unsafe {
            __switch(&_unused as *const _, next_task_cx_ptr2);
        }
    }

    fn mark_current_suspended(&self) {
        let mut inner = self.inner.borrow_mut();
        let current = inner.current_task;
        inner.tasks[current].duration += get_time_ms() - inner.current_start;
        if inner.tasks[current].duration >= MAX_DURATION {
            debug!("Kill {}", current);
            inner.tasks[current].task_status = TaskStatus::Exited;
        } else {
            inner.tasks[current].task_status = TaskStatus::Ready;
        }
    }

    fn mark_current_exited(&self) {
        let mut inner = self.inner.borrow_mut();
        let current = inner.current_task;
        inner.tasks[current].duration += get_time_ms() - inner.current_start;
        inner.tasks[current].task_status = TaskStatus::Exited;
    }

    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.borrow();
        let current = inner.current_task;
        (current + 1..current + self.num_app + 1)
            .filter_map(
                |id: usize| match inner.tasks[id % self.num_app].task_status {
                    TaskStatus::Ready => Some(id % self.num_app),
                    _ => None,
                },
            )
            .min_by(|x: &usize, y: &usize| {
                inner.tasks[*x]
                    .stride
                    .partial_cmp(&inner.tasks[*y].stride)
                    .unwrap()
            })
    }

    fn run_next_task(&self) {
        if let Some(next) = self.find_next_task() {
            let mut inner = self.inner.borrow_mut();
            let current = inner.current_task;
            inner.tasks[next].task_status = TaskStatus::Running;
            let priority = inner.tasks[next].priority;
            inner.tasks[next].stride.update(priority);
            inner.current_start = get_time_ms();
            inner.current_task = next;
            let current_task_cx_ptr2 = inner.tasks[current].get_task_cx_ptr2();
            let next_task_cx_ptr2 = inner.tasks[next].get_task_cx_ptr2();
            core::mem::drop(inner);
            trace!(
                "current {:?} next {:?}",
                current_task_cx_ptr2,
                next_task_cx_ptr2
            );
            unsafe {
                __switch(current_task_cx_ptr2, next_task_cx_ptr2);
            }
        } else {
            panic!("All applications completed!");
        }
    }

    fn set_current_priority(&self, priority: isize) -> Result<isize, isize> {
        if priority <= 1 {
            Err(-1)
        } else {
            let mut inner = self.inner.borrow_mut();
            let current = inner.current_task;
            inner.tasks[current].priority = priority;
            Ok(priority)
        }
    }

    fn check_read_memory(&self, buf: *const u8, len: usize) -> Result<(), i32> {
        let pc = sepc::read();
        let upc = sscratch::read();
        let max_mem_bound = (pc + APP_SIZE_LIMIT - 1) & (!(APP_SIZE_LIMIT - 1));
        let min_mem_bound = max_mem_bound - APP_SIZE_LIMIT;
        let max_stack_bound = (upc + USER_STACK_SIZE - 1) & (!(USER_STACK_SIZE - 1));
        let min_stack_bound = max_stack_bound - USER_STACK_SIZE;

        if (min_mem_bound > buf as usize || max_mem_bound < buf as usize + len)
            && (min_stack_bound > buf as usize || max_stack_bound < buf as usize + len)
        {
            return Err(-1);
        }
        Ok(())
    }
}

pub fn run_first_task() {
    TASK_MANAGER.run_first_task();
}

fn run_next_task() {
    TASK_MANAGER.run_next_task();
}

fn mark_current_suspended() {
    TASK_MANAGER.mark_current_suspended();
}

fn mark_current_exited() {
    TASK_MANAGER.mark_current_exited();
}

pub fn suspend_current_and_run_next() {
    mark_current_suspended();
    run_next_task();
}

pub fn exit_current_and_run_next() {
    mark_current_exited();
    run_next_task();
}

pub fn set_current_priority(priority: isize) -> Result<isize, isize> {
    TASK_MANAGER.set_current_priority(priority)
}

pub fn check_read_memory(buf: *const u8, len: usize) -> Result<(), i32> {
    TASK_MANAGER.check_read_memory(buf, len)
}
