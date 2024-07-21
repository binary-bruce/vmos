mod address;
mod frame;
mod heap;
mod paging;
mod vm;

/// init heap allocator and frame allocator
pub fn init() {
    heap::init_heap_allocator();
    println!("[kernel][mm] Initiated heap allocator");

    frame::init_frame_allocator();
    println!("[kernel][mm] Initiated frame allocator");

    vm::enable_virtual_memory();
    println!("[kernel][mm] Enabled virtual memory");
}
