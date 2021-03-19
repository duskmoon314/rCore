use crate::{
    task::{exit_current_and_run_next, set_current_priority, suspend_current_and_run_next},
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

pub fn sys_get_time(time: &mut TimeVal, tz: usize) -> isize {
    get_time(time, tz)
}
