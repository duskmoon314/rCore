mod inode;
mod mail;
mod pipe;
mod stdio;

use crate::mm::UserBuffer;

pub use mail::{MailBox, Socket};
pub use pipe::{make_pipe, Pipe};
pub use stdio::{Stdin, Stdout};
pub trait File: Send + Sync {
    fn readable(&self) -> bool;
    fn writable(&self) -> bool;
    fn read(&self, buf: UserBuffer) -> Result<usize, isize>;
    fn write(&self, buf: UserBuffer) -> Result<usize, isize>;
    fn stat(&self, st: &mut Stat) -> Result<usize, isize>;
}

pub use inode::{link_at, list_apps, open_file, unlink_at, OSInode, OpenFlags, Stat};
