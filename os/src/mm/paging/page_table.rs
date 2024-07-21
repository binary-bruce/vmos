use alloc::vec::Vec;

use crate::mm::{
    address::ppn::PhysPageNum,
    frame::{frame_alloc, FrameTracker},
};

pub struct PageTable {
    root_ppn: PhysPageNum,

    /// the allocated physical frames
    /// got dropped if PageTable is dropped based on RAII
    frames: Vec<FrameTracker>,
}

impl PageTable {
    /// allocate a physical frame as the root page table
    pub fn new() -> Self {
        let root = frame_alloc().unwrap();
        PageTable {
            root_ppn: root.ppn,
            frames: alloc::vec![root],
        }
    }
}
