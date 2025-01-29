use alloy_consensus::Header;
use alloy_primitives::{keccak256, B256};
use alloy_rlp::Encodable;
use auto_impl::auto_impl;

#[auto_impl(&)]
pub trait Hashable {
    /// Calculate the hash, this may be slow.
    fn hash_slow(&self) -> B256;
}

impl Hashable for Header {
    fn hash_slow(&self) -> B256 {
        let mut out = Vec::<u8>::new();
        self.encode(&mut out);
        keccak256(&out)
    }
}
