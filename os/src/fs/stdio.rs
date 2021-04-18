use super::{File, Stat};
use crate::mm::UserBuffer;
use crate::sbi::console_getchar;
use crate::task::suspend_current_and_run_next;

pub struct Stdin;

pub struct Stdout;

impl File for Stdin {
    fn readable(&self) -> bool {
        true
    }
    fn writable(&self) -> bool {
        false
    }
    fn read(&self, mut user_buf: UserBuffer) -> Result<usize, isize> {
        assert_eq!(user_buf.len(), 1);
        // busy loop
        let mut c: usize;
        loop {
            c = console_getchar();
            if c == 0 {
                suspend_current_and_run_next();
                continue;
            } else {
                break;
            }
        }
        let ch = c as u8;
        unsafe {
            user_buf.buffers[0].as_mut_ptr().write_volatile(ch);
        }
        Ok(1)
    }
    fn write(&self, _user_buf: UserBuffer) -> Result<usize, isize> {
        panic!("Cannot write to stdin!");
    }

    fn stat(&self, st: &mut Stat) -> Result<usize, isize> {
        *st = Stat::new();
        Ok(0)
    }
}

impl File for Stdout {
    fn readable(&self) -> bool {
        false
    }
    fn writable(&self) -> bool {
        true
    }
    fn read(&self, _user_buf: UserBuffer) -> Result<usize, isize> {
        panic!("Cannot read from stdout!");
    }
    fn write(&self, user_buf: UserBuffer) -> Result<usize, isize> {
        for buffer in user_buf.buffers.iter() {
            print!("{}", core::str::from_utf8(*buffer).unwrap());
        }
        Ok(user_buf.len())
    }

    fn stat(&self, st: &mut Stat) -> Result<usize, isize> {
        *st = Stat::new();
        Ok(0)
    }
}
