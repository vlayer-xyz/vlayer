use bytes::Bytes;
use derive_new::new;
use risc0_zkp::core::digest::Digest;

#[derive(Debug, Clone, new, PartialEq, PartialOrd, Eq, Ord, Default)]
pub struct Guest {
    pub id: Digest,
    pub elf: Bytes,
}
