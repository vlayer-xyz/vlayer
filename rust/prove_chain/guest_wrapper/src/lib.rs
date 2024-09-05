#[cfg(not(clippy))]
include!(concat!(env!("OUT_DIR"), "/methods.rs"));

#[cfg(clippy)]
pub const RISC0_PROVE_CHAIN_GUEST_ELF: &[u8] = &[];

#[cfg(clippy)]
pub const RISC0_PROVE_CHAIN_GUEST_ID: [u32; 8] = [0; 8];
