use crate::mm::translated_byte_buffer;
// use crate::task::check_read_memory;
use crate::task::current_user_token;

const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    // if let Err(_cause) = check_read_memory(buf, len) {
    //     return -1;
    // }
    match fd {
        FD_STDOUT => {
            let buffers = translated_byte_buffer(current_user_token(), buf, len);
            for buffer in buffers {
                print!("{}", core::str::from_utf8(buffer).unwrap());
            }
            len as isize
        }
        _ => {
            error!("Unsupported fd {} in sys_write!", fd);
            -1
        }
    }
}
