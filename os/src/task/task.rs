use core::cmp::Ordering;

use crate::config::BIG_STRIDE;

pub struct TaskControlBlock {
    pub task_cx_ptr: usize,
    pub task_status: TaskStatus,
    pub stride: Stride,
    pub priority: isize,
    pub duration: usize,
}

impl TaskControlBlock {
    pub fn get_task_cx_ptr2(&self) -> *const usize {
        &self.task_cx_ptr as *const usize
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}

#[derive(PartialEq)]
pub struct Stride(pub usize);

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        ((self.0 - other.0) as isize).partial_cmp(&0)
    }
}

impl Stride {
    pub fn update(&mut self, priority: isize) {
        if priority <= 1 {
            panic!("Priority must larger then 1");
        } else {
            self.0 += BIG_STRIDE / priority as usize;
        }
    }
}
