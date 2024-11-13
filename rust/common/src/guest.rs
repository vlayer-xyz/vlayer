use bytes::Bytes;
use risc0_zkp::core::digest::Digest;

pub struct Guest {
    pub id: Digest,
    pub elf: Bytes,
}
