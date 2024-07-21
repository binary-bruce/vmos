use core::fmt::{Debug, Formatter, Result};

use super::address::ppn::PhysPageNum;

/// manage the lifecycle of the contained ppn based on RAII
pub struct FrameTracker {
    pub ppn: PhysPageNum,
}

impl FrameTracker {
    pub fn new(ppn: PhysPageNum) -> Self {
        let bytes_array = ppn.get_bytes_array();
        for i in bytes_array {
            *i = 0;
        }
        Self { ppn }
    }
}

impl Debug for FrameTracker {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_fmt(format_args!("FrameTracker:PPN={:#x}", self.ppn.0))
    }
}

impl Drop for FrameTracker {
    fn drop(&mut self) {
        todo!()
    }
}
