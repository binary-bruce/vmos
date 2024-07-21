use riscv::register::time;

use crate::{config::CLOCK_FREQ, sbi::set_timer};

const TICKS_PER_SEC: usize = 100;
const MSEC_PER_SEC: usize = 1000;

/// read the `mtime` register
#[inline(always)]
pub fn get_time() -> usize {
    time::read()
}

/// get current time in milliseconds
pub fn get_time_ms() -> usize {
    get_time() / (CLOCK_FREQ / MSEC_PER_SEC)
}

/// set the next timer interrupt
pub fn set_next_timer() {
    set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC) // next 10 milliseconds
}
