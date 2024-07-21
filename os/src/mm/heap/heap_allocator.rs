use buddy_system_allocator::LockedHeap;

/// The const parameter of LockedHeap is the max order of the allocator; i.e. in this case it can allocate regions of up to 2^32 bytes.
#[global_allocator]
pub(crate) static HEAP_ALLOCATOR: LockedHeap<32> = LockedHeap::empty();

// #[alloc_error_handler]
// pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
//     panic!("Heap allocation error, layout = {:?}", layout);
// }
