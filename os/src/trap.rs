//! Trap handling functionality
//!
//! For rCore, we have a single trap entry point, namely `__alltraps`. At
//! initialization in [`init()`], we set the `stvec` CSR to point to it.
//!
//! All traps go through `__alltraps`, which is defined in `trap.S`. The
//! assembly language code does just enough work restore the kernel space
//! context, ensuring that Rust code safely runs, and transfers control to
//! [`trap_handler()`].
//!
//! It then calls different functionality based on what exactly the exception
//! was. For example, timer interrupts trigger task preemption, and syscalls go
//! to [`syscall()`].

/// Trap Context
mod context;
pub use context::TrapContext;

use crate::{syscall::syscall, task::suspend_current_and_run_next, timer::set_next_timer};
use core::arch::global_asm;
use riscv::register::{
    scause::Interrupt,
    scause::{self, Exception, Trap},
    sie, stval, stvec,
};

global_asm!(include_str!("trap/trap.S"));

/// init CSR `stvec` as the entry of `__alltraps`
pub fn init() {
    extern "C" {
        fn __alltraps();
    }

    unsafe { stvec::write(__alltraps as usize, stvec::TrapMode::Direct) };
}

/// enable timer interrupt
pub fn enable_timer_interrupt() {
    unsafe {
        sie::set_stimer();
    }
}

#[no_mangle]
/// entry point to handle interrupt, exception or system call from user space
pub(crate) fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read(); // trap cause
    let stval = stval::read(); // extra value of the trap

    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4; // move forward to next instruction
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            println!("[kernel] PageFault in application, kernel killed it.");
            panic!("[kernel] Cannot continue!");
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, kernel killed it.");
            println!("");
            panic!("[kernel] Cannot continue!");
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            set_next_timer();
            suspend_current_and_run_next();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }

    cx
}
