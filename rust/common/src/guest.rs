use bytes::Bytes;
use risc0_zkp::core::digest::Digest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Default, Serialize, Deserialize)]
pub struct Guest {
    pub id: Digest,
    pub elf: Bytes,
}

impl Guest {
    pub fn new(id: impl Into<Digest>, elf: impl Into<Bytes>) -> Self {
        Self {
            id: id.into(),
            elf: elf.into(),
        }
    }
}
