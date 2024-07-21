use alloc::vec::Vec;

use crate::mm::{
    address::{PhysPageNum, VirtPageNum},
    frame::{frame_alloc, FrameTracker},
};

use super::{PTEFlags, PageTableEntry};

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

    /// Temporarily used to constrct the user space page table
    /// Hence it does not own the frames
    pub fn from_token(satp: usize) -> Self {
        let root = satp & (1usize << 44 - 1);
        Self {
            root_ppn: PhysPageNum::from(root),
            frames: Vec::new(),
        }
    }

    pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PTEFlags) {
        let pte = self.find_pte_create(vpn).unwrap();
        assert!(!pte.is_valid(), "vpn {:?} is mapped before mapping", vpn);

        *pte = PageTableEntry::new(ppn, flags | PTEFlags::V);
    }

    pub fn unmap(&mut self, vpn: VirtPageNum) {
        let pte = self.find_pte(vpn).unwrap();
        assert!(pte.is_valid(), "vpn {:?} is invalid before unmapping", vpn);

        *pte = PageTableEntry::empty();
    }

    pub fn tranlate(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        self.find_pte(vpn).map(|pte| *pte)
    }

    fn find_pte(&self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let indexes = vpn.indexes();
        let mut ppn = self.root_ppn;
        let mut result: Option<&mut PageTableEntry> = None;

        for (i, v) in indexes.iter().enumerate() {
            let pte = &mut ppn.get_pte_array()[*v]; // v is the page index
            if i == 2 {
                result = Some(pte);
                break;
            }

            if !pte.is_valid() {
                return None;
            }

            ppn = pte.ppn()
        }

        result
    }

    fn find_pte_create(&mut self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let indexes = vpn.indexes();
        let mut ppn = self.root_ppn;
        let mut result: Option<&mut PageTableEntry> = None;

        for (i, v) in indexes.iter().enumerate() {
            let pte = &mut ppn.get_pte_array()[*v]; // v is the page index
            if i == 2 {
                result = Some(pte);
                break;
            }

            // need to create if it's not a valid page entry
            if !pte.is_valid() {
                let frame = frame_alloc().unwrap();
                *pte = PageTableEntry::new(frame.ppn, PTEFlags::V);
                self.frames.push(frame);
                return None;
            }

            ppn = pte.ppn()
        }

        result
    }
}
