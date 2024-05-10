use lighthouse_common::LEB128Vec;

pub mod assert;
pub mod write;

pub type CompactBytes = LEB128Vec<u8>;
