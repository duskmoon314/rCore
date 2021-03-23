use crate::mm::translated_byte_buffer;
use crate::task::current_user_token;
use crate::task::set_current_priority;
use core::mem::size_of;

use crate::{
    task::{exit_current_and_run_next, suspend_current_and_run_next},
    timer::{get_time, TimeVal},
};

pub fn sys_exit(exit_code: i32) -> ! {
    debug!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

pub fn sys_set_priority(prio: isize) -> isize {
    match set_current_priority(prio) {
        Ok(prio) => prio,
        Err(err) => err,
    }
}

// Need to be fixed
pub fn sys_get_time(time: &mut TimeVal, tz: usize) -> isize {
    get_time(time, tz)
}
