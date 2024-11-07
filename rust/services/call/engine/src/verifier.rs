mod chain_proof;
pub mod guest_input;
mod zk_proof;

#[cfg(test)]
mod tests;

pub use chain_proof::{ChainProofError, ChainProofVerifier, ZkChainProofVerifier};
pub use zk_proof::{GuestVerifier, HostVerifier, VerificationError, ZkpVerifier};
