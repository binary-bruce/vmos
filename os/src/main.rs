//! The main module and entrypoint
//!
//! Various facilities of the kernels are implemented as submodules. The most
//! important ones are:
//!
//! - [`trap`]: Handles all cases of switching from userspace to the kernel
//! - [`task`]: Task management
//! - [`syscall`]: System call handling and implementation
//!
//! The operating system also starts in this module. Kernel code starts
//! executing from `entry.asm`, after which [`rust_main()`] is called to
//! initialize various pieces of functionality. (See its source code for
//! details.)
//!
//! We then call [`task::run_first_task()`] and for the first time go to
//! userspace.

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]
#![no_main]
#![feature(panic_info_message)]

extern crate alloc;

use core::arch::global_asm;

#[path = "boards/qemu.rs"]
mod board;

#[macro_use]
mod console;
mod config;
mod lang_items;
mod loader;
mod mm;
mod sbi;
mod stack;
mod sync;
mod syscall;
mod task;
mod timer;
mod trap;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

/// clear BSS
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }

    unsafe {
        let start = sbss as usize as *mut u8;
        let len = ebss as usize - sbss as usize;
        core::slice::from_raw_parts_mut(start, len).fill(0);
    }
}

/// the entry point of os
#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("[kernel] Cleared bss segment");

    trap::init();
    println!("[kernel] Initiated trap handler");

    trap::enable_timer_interrupt();
    println!("[kernel] Enabled timer interrupt");

    timer::set_next_timer();
    println!("[kernel] Set timer");

    loader::load_apps();
    println!("[kernel] Loaded tasks");

    println!("[kernel] Starting to run first task");
    task::run_first_task();

    panic!("Unreachable in rust_main!");
}
