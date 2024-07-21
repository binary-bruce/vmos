//! physical page number

use core::fmt::{Debug, Formatter, Result};

use super::{pa::PhysAddr, PPN_WIDTH_SV39};

/// physical page number
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq)]
pub struct PhysPageNum(pub usize);

impl Debug for PhysPageNum {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_fmt(format_args!("PPN:{:#x}", self.0))
    }
}

impl From<usize> for PhysPageNum {
    fn from(v: usize) -> Self {
        Self(v & ((1 << PPN_WIDTH_SV39) - 1))
    }
}

impl From<PhysPageNum> for usize {
    fn from(v: PhysPageNum) -> Self {
        v.0
    }
}

impl From<PhysAddr> for PhysPageNum {
    fn from(v: PhysAddr) -> Self {
        assert_eq!(v.page_offset(), 0);
        v.floor()
    }
}
