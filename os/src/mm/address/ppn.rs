//! physical page number

use core::fmt::{Debug, Formatter, Result};

use crate::mm::paging::PageTableEntry;

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

impl PhysPageNum {
    pub fn get_pte_array(&self) -> &'static mut [PageTableEntry] {
        let pa: PhysAddr = (*self).into();
        let ptr = pa.0 as *mut PageTableEntry;
        unsafe { core::slice::from_raw_parts_mut(ptr, 512) }
    }

    pub fn get_bytes_array(&self) -> &'static mut [u8] {
        let pa: PhysAddr = (*self).into();
        let ptr = pa.0 as *mut u8;
        unsafe { core::slice::from_raw_parts_mut(ptr, 4096) }
    }

    pub fn get_mut<T>(&self) -> &'static mut T {
        let pa: PhysAddr = (*self).into();
        let ptr = pa.0 as *mut T;
        unsafe { ptr.as_mut().unwrap() }
    }
}
