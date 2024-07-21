use alloc::vec::Vec;

use super::address::ppn::PhysPageNum;

trait FrameAllocator {
    fn new() -> Self;

    fn alloc(&mut self) -> Option<PhysPageNum>;

    fn dealloc(&mut self, ppn: PhysPageNum);
}

pub struct StackFrameAllocator {
    current: usize,
    end: usize,

    // Why can we use Vec that requires dynamic memory management??
    //
    // Because in the init phase, there is no actually dynamic memory allocation yet,
    // only a variable(with some internal fields: buf, len) allocted in data segment
    recycled: Vec<usize>,
}

impl StackFrameAllocator {
    pub fn init(&mut self, start: PhysPageNum, end: PhysPageNum) {
        self.current = start.0;
        self.end = end.0;
    }
}

impl FrameAllocator for StackFrameAllocator {
    fn new() -> Self {
        Self {
            current: 0,
            end: 0,
            recycled: Vec::new(),
        }
    }

    fn alloc(&mut self) -> Option<PhysPageNum> {
        if let Some(ppn) = self.recycled.pop() {
            Some(ppn.into())
        } else if self.current == self.end {
            None
        } else {
            self.current += 1;
            Some((self.current - 1).into())
        }
    }

    fn dealloc(&mut self, ppn: PhysPageNum) {
        let ppn = ppn.0;
        if ppn >= self.current || self.recycled.iter().any(|v| *v == ppn) {
            panic!("Frame ppn={:#x} has not been allocated!", ppn);
        }
        self.recycled.push(ppn);
    }
}
