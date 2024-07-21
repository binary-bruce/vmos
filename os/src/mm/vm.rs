use alloc::sync::Arc;
use lazy_static::lazy_static;

pub use map_area::*;
pub use map_permission::*;
pub use map_type::*;
pub use memory_set::*;

use crate::sync::UPSafeCell;

mod map_area;
mod map_permission;
mod map_type;
mod memory_set;

lazy_static! {
    pub static ref KERNEL_SPACE: Arc<UPSafeCell<MemorySet>> = {
        let kernel_space = MemorySet::new_kernel();
        Arc::new(unsafe { UPSafeCell::new(kernel_space) })
    };
}

pub fn enable_virtual_memory() {
    KERNEL_SPACE.exclusive_access().activate();
}
