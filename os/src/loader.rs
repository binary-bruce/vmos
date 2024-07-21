//! Loading user applications into memory

use core::arch::asm;

use crate::{
    config::{APP_BASE_ADDRESS, APP_SIZE_LIMIT, KERNEL_STACK_SIZE, MAX_APP_NUM, USER_STACK_SIZE},
    stack,
    trap::TrapContext,
};
use stack::{KernelStack, UserStack};

static KERNER_STACK: [KernelStack; MAX_APP_NUM] = [KernelStack {
    data: [0; KERNEL_STACK_SIZE],
}; MAX_APP_NUM];

static USER_STACK: [UserStack; MAX_APP_NUM] = [UserStack {
    data: [0; USER_STACK_SIZE],
}; MAX_APP_NUM];

fn get_base_i(app_id: usize) -> usize {
    APP_BASE_ADDRESS + app_id * APP_SIZE_LIMIT
}

/// Get the total number of applications.
pub fn get_num_app() -> usize {
    extern "C" {
        fn _num_app();
    }
    unsafe { (_num_app as usize as *const usize).read_volatile() }
}

pub fn load_apps() {
    extern "C" {
        fn _num_app();
    }

    let num_app_ptr = _num_app as usize as *const usize;
    let num_app = get_num_app();
    let app_start = unsafe { core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1) };

    // load apps
    for i in 0..num_app {
        let base_i = get_base_i(i);
        println!("app{} base: {}", i, base_i);

        // clear region
        (base_i..base_i + APP_SIZE_LIMIT)
            .for_each(|addr| unsafe { (addr as *mut u8).write_volatile(0) });

        // load app from data section to memory
        let src = unsafe {
            core::slice::from_raw_parts(app_start[i] as *const u8, app_start[i + 1] - app_start[i])
        };
        let dst = unsafe { core::slice::from_raw_parts_mut(base_i as *mut u8, src.len()) };
        dst.copy_from_slice(src);
    }

    unsafe {
        asm!("fence.i");
    }
}

/// get app info with entry and sp and save `TrapContext` in kernel stack
pub fn init_trap_cx(app_id: usize) -> usize {
    let trap_cx = TrapContext::app_init_context(get_base_i(app_id), USER_STACK[app_id].get_sp());
    KERNER_STACK[app_id].push_context(trap_cx)
}
