use frame_allocator::FRAME_ALLOCATOR;
use frame_tracker::FrameTracker;

use crate::{config::MEMORY_END, mm::address::pa::PhysAddr};

use super::address::ppn::PhysPageNum;

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
