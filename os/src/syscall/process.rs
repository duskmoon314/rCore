use crate::mm::translate_writable_va;
use crate::task::current_user_token;
use crate::task::set_current_priority;
use crate::task::{mmap, munmap};
use alloc::vec::Vec;
use core::mem::size_of;

use crate::{
    task::{exit_current_and_run_next, suspend_current_and_run_next},
    timer::get_time,
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

pub fn sys_get_time(time: usize, tz: usize) -> isize {
    let token = current_user_token();
    let mut pas: Vec<*mut usize> = Vec::new();
    match translate_writable_va(token, time) {
        Err(_) => return -1,
        Ok(pa) => pas.push(pa as *mut usize),
    }
    match translate_writable_va(token, time + size_of::<usize>()) {
        Err(_) => return -1,
        Ok(pa) => pas.push(pa as *mut usize),
    }
    get_time(pas, tz)
}

pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    match mmap(start, len, port) {
        Ok(map_size) => map_size,
        Err(_) => -1,
    }
}

pub fn sys_munmap(start: usize, len: usize) -> isize {
    match munmap(start, len) {
        Ok(len) => len,
        Err(_) => -1,
    }
}
