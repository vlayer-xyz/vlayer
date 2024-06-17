#[cfg(not(clippy))]
include!(concat!(env!("OUT_DIR"), "/methods.rs"));

#[cfg(clippy)]
pub const GUEST_ELF: &[u8] = &[];
