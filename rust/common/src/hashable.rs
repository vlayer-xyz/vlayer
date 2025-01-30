use alloy_primitives::B256;
use auto_impl::auto_impl;

#[auto_impl(&)]
pub trait Hashable {
    /// Calculate the hash, this may be slow.
    fn hash_slow(&self) -> B256;
}
