use frame_allocator::FRAME_ALLOCATOR;
pub use frame_tracker::FrameTracker; // re-export it

use crate::{config::MEMORY_END, mm::address::PhysAddr};

use super::address::PhysPageNum;

mod frame_allocator;
mod frame_tracker;

pub fn init_frame_allocator() {
    extern "C" {
        fn ekernel();
    }

    FRAME_ALLOCATOR.exclusive_access().init(
        PhysAddr::from(ekernel as usize).ceil(),
        PhysAddr::from(MEMORY_END).floor(),
    )
}

pub fn frame_alloc() -> Option<FrameTracker> {
    frame_allocator::frame_alloc()
}

pub fn frame_dealloc(ppn: PhysPageNum) {
    frame_allocator::frame_dealloc(ppn)
}
