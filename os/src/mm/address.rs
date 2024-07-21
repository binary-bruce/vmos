use crate::config::PAGE_SIZE_BITS;

pub mod pa;
pub mod ppn;

const PA_WIDTH_SV39: usize = 56;
const PPN_WIDTH_SV39: usize = PA_WIDTH_SV39 - PAGE_SIZE_BITS; // 56 - 12 == 44
