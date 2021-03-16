const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    // if let Err(_cause) = check_read_memory(buf, len) {
    //     return -1;
    // }
    match fd {
        FD_STDOUT => {
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            len as isize
        }
        _ => {
            error!("Unsupported fd {} in sys_write!", fd);
            -1
        }
    }
}
