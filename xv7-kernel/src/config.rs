/// Maximum pyhsical memory that we decided to support.
#[allow(unused)]
pub const MAX_PHYSICAL_ADDRESS_SUPPORTED: usize = 32;
/// Maximum number of pyhsical frames calculated from `MAX_PHYSICAL_ADDRESS_SUPPORTED`.
#[allow(unused)]
pub const MAX_FRAMES_SUPPORTED: usize = 1 << MAX_PHYSICAL_ADDRESS_SUPPORTED >> 12;
