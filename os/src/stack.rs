use crate::{
    config::{KERNEL_STACK_SIZE, USER_STACK_SIZE},
    trap::TrapContext,
};

#[repr(align(4096))]
#[derive(Copy, Clone)]
pub(crate) struct KernelStack {
    pub(crate) data: [u8; KERNEL_STACK_SIZE],
}

#[repr(align(4096))]
#[derive(Copy, Clone)]
pub(crate) struct UserStack {
    pub(crate) data: [u8; USER_STACK_SIZE],
}

impl KernelStack {
    pub(crate) fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }

    /// return the pointer of the trap context in kernel stack
    pub(crate) fn push_context(&self, trap_cx: TrapContext) -> usize {
        let trap_cx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *trap_cx_ptr = trap_cx;
        }

        trap_cx_ptr as usize
    }
}

impl UserStack {
    pub(crate) fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}
