#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(llvm_asm)]
#![feature(panic_info_message)]

#[allow(unused_imports)]
use console::ANSICON;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

#[macro_use]
mod console;
mod batch;
mod lang_items;
mod logger;
mod sbi;
mod syscall;
mod trap;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

#[no_mangle]
pub fn rust_main() -> ! {
    logger::init();
    clear_bss();
    println!("[kernel] Hello, world!");
    trap::init();
    batch::init();
    batch::run_next_app();
}
