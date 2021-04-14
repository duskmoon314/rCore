use core::cmp::min;

use crate::fs::{make_pipe, open_file, File, OpenFlags};
use crate::mm::{translated_byte_buffer, translated_refmut, translated_str, UserBuffer};
use crate::task::{current_task, current_user_token, find_task};
use alloc::sync::Arc;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.acquire_inner_lock();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd] {
        if !file.writable() {
            return -1;
        }
        let file = file.clone();
        // release Task lock manually to avoid deadlock
        drop(inner);
        if let Ok(buffers) = translated_byte_buffer(token, buf, len) {
            match file.write(UserBuffer::new(buffers)) {
                Ok(write_len) => write_len as isize,
                Err(_) => -1,
            }
        } else {
            -1
        }
    } else {
        -1
    }
}

pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.acquire_inner_lock();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd] {
        let file = file.clone();
        if !file.readable() {
            return -1;
        }
        // release Task lock manually to avoid deadlock
        drop(inner);
        if let Ok(buffers) = translated_byte_buffer(token, buf, len) {
            match file.read(UserBuffer::new(buffers)) {
                Ok(read_len) => read_len as isize,
                Err(_) => -1,
            }
        } else {
            -1
        }
    } else {
        -1
    }
}

pub fn sys_open(path: *const u8, flags: u32) -> isize {
    let task = current_task().unwrap();
    let token = current_user_token();
    let path = translated_str(token, path);
    if let Some(inode) = open_file(path.as_str(), OpenFlags::from_bits(flags).unwrap()) {
        let mut inner = task.acquire_inner_lock();
        let fd = inner.alloc_fd();
        inner.fd_table[fd] = Some(inode);
        fd as isize
    } else {
        -1
    }
}

pub fn sys_close(fd: usize) -> isize {
    let task = current_task().unwrap();
    let mut inner = task.acquire_inner_lock();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if inner.fd_table[fd].is_none() {
        return -1;
    }
    inner.fd_table[fd].take();
    0
}

pub fn sys_pipe(pipe: *mut usize) -> isize {
    let task = current_task().unwrap();
    let token = current_user_token();
    let mut inner = task.acquire_inner_lock();
    let (pipe_read, pipe_write) = make_pipe();
    let read_fd = inner.alloc_fd();
    inner.fd_table[read_fd] = Some(pipe_read);
    let write_fd = inner.alloc_fd();
    inner.fd_table[write_fd] = Some(pipe_write);
    *translated_refmut(token, pipe) = read_fd;
    *translated_refmut(token, unsafe { pipe.add(1) }) = write_fd;
    0
}

pub fn sys_mailwrite(pid: usize, buf: *mut u8, len: usize) -> isize {
    let token = current_user_token();
    if let Some(receive_task) = find_task(pid) {
        debug!("find task");
        if receive_task.acquire_inner_lock().is_mailbox_full() {
            return -1;
        } else if len == 0 {
            return 0;
        }

        if let Ok(buffers) = translated_byte_buffer(token, buf, min(len, 256)) {
            let socket = receive_task.create_socket();
            match socket.write(UserBuffer::new(buffers)) {
                Ok(write_len) => return write_len as isize,
                Err(_) => return -1,
            }
        } else {
            return -1;
        }
    } else {
        debug!("not find task");
        -1
    }
}

pub fn sys_mailread(buf: *mut u8, len: usize) -> isize {
    let token = current_user_token();
    let task = current_task().unwrap();
    debug!(
        "mail box empty ? {}",
        task.acquire_inner_lock().is_mailbox_empty()
    );
    if task.acquire_inner_lock().is_mailbox_empty() {
        return -1;
    } else if len == 0 {
        return 0;
    }
    let mail_box = task.acquire_inner_lock().mail_box.clone();
    if let Ok(buffers) = translated_byte_buffer(token, buf, min(len, 256)) {
        match mail_box.read(UserBuffer::new(buffers)) {
            Ok(read_len) => {
                debug!("mail read {} len", read_len);
                read_len as isize
            }
            Err(_) => -1,
        }
    } else {
        -1
    }
}

pub fn sys_dup(fd: usize) -> isize {
    let task = current_task().unwrap();
    let mut inner = task.acquire_inner_lock();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if inner.fd_table[fd].is_none() {
        return -1;
    }
    let new_fd = inner.alloc_fd();
    inner.fd_table[new_fd] = Some(Arc::clone(inner.fd_table[fd].as_ref().unwrap()));
    new_fd as isize
}
