use bytes::Bytes;
use derive_more::AsRef;
use risc0_zkp::core::digest::Digest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize, AsRef, Default)]
pub struct GuestElf {
    #[as_ref]
    pub id: Digest,
    #[as_ref]
    pub elf: Bytes,
}

impl GuestElf {
    pub const fn new(id: [u32; 8], elf: &'static [u8]) -> Self {
        Self {
            id: Digest::new(id),
            elf: Bytes::from_static(elf),
        }
    }

    pub const fn default() -> Self {
        Self::new([0; 8], &[])
    }
}
