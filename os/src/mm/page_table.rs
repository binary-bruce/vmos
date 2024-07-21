use bitflags::bitflags;

use super::address::ppn::PhysPageNum;

bitflags! {
    pub struct PTEFlags: u8 {
        const V = 1 << 0;
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
        const G = 1 << 5;
        const A = 1 << 6;
        const D = 1 << 7;
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
/// page table entry structure
pub struct PageTableEntry {
    pub bits: usize,
}

impl PageTableEntry {
    const FLAGS_BITS: usize = 10; // Why 10 instead of 8? 2 bits are reserved

    pub fn new(ppn: PhysPageNum, flags: PTEFlags) -> Self {
        PageTableEntry {
            bits: ppn.0 << Self::FLAGS_BITS | flags.bits() as usize,
        }
    }

    pub fn empty() -> Self {
        Self { bits: 0 }
    }

    pub fn ppn(&self) -> PhysPageNum {
        ((self.bits >> Self::FLAGS_BITS) & (1usize << 44 - 1)).into()
    }

    pub fn flags(&self) -> PTEFlags {
        PTEFlags::from_bits(self.bits as u8).unwrap()
    }

    pub fn is_valid(&self) -> bool {
        self.flags().contains(PTEFlags::V)
    }

    pub fn readable(&self) -> bool {
        self.flags().contains(PTEFlags::R)
    }

    pub fn writable(&self) -> bool {
        self.flags().contains(PTEFlags::W)
    }

    pub fn executable(&self) -> bool {
        self.flags().contains(PTEFlags::X)
    }
}
