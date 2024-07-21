mod address;
mod frame;
mod heap;
mod paging;

/// init heap allocator and frame allocator
pub fn init() {
    heap::init_heap_allocator();
    frame::init_frame_allocator();
}
