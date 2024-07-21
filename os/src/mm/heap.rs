use heap_allocator::HEAP_ALLOCATOR;

use crate::config::KERNEL_HEAP_SIZE;

mod heap_allocator;

/// heap space ([u8; KERNEL_HEAP_SIZE])
/// in BSS segment
static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

/// init heap allocator
pub fn init_heap_allocator() {
    unsafe {
        HEAP_ALLOCATOR
            .lock()
            .init(HEAP_SPACE.as_ptr() as usize, KERNEL_HEAP_SIZE)
    }
}
