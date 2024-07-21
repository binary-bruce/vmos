use crate::config::PAGE_SIZE_BITS;

pub mod pa;
pub mod ppn;
pub mod va;
pub mod vpn;

const PA_WIDTH_SV39: usize = 56;
const PPN_WIDTH_SV39: usize = PA_WIDTH_SV39 - PAGE_SIZE_BITS; // 56 - 12 == 44

const VA_WIDTH_SV39: usize = 39; // 3 * 9 + 12
const VPN_WIDTH_SV39: usize = VA_WIDTH_SV39 - PAGE_SIZE_BITS; // 27
