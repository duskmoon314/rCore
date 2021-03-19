use crate::config::CLOCK_FREQ;
use crate::sbi::set_timer;
use riscv::register::time;

const TICKS_PER_SEC: usize = 100;
const MSEC_PER_SEC: usize = 1000;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

#[allow(dead_code)]
impl TimeVal {
    pub fn new() -> Self {
        TimeVal { sec: 0, usec: 0 }
    }
}

#[allow(unused_variables)]
pub fn get_time(ts: &mut TimeVal, tz: usize) -> isize {
    let t = time::read();
    ts.sec = t / CLOCK_FREQ;
    ts.usec = (t % CLOCK_FREQ) * 1000000 / CLOCK_FREQ;
    trace!("t {} sec {} usec {}", t, ts.sec, ts.usec);
    0
}

#[allow(dead_code)]
pub fn get_time_ms() -> usize {
    time::read() / (CLOCK_FREQ / MSEC_PER_SEC)
}

pub fn set_next_trigger() {
    set_timer(time::read() + CLOCK_FREQ / TICKS_PER_SEC);
}
