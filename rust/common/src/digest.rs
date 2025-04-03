use std::fmt::{Display, Formatter};

use bytemuck::{Pod, PodCastError, Zeroable};
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Serialize,
    Deserialize,
    Default,
    Pod,
    Zeroable,
)]
#[repr(transparent)]
pub struct Digest([u32; 8]);

impl Digest {
    /// Digest of all zeroes.
    pub const ZERO: Self = Self::new([0_u32; 8]);

    pub const fn new(digest: [u32; 8]) -> Self {
        Self(digest)
    }

    pub fn as_bytes(&self) -> &[u8] {
        bytemuck::cast_slice(&self.0)
    }

    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        let mut digest: Digest = Digest::ZERO;
        let mut i: usize = 0;
        while i < 8 {
            let mut j = 0;
            let mut word = 0_u32;
            while j < 4 {
                word <<= 8;
                word |= bytes[i * 4 + j] as u32;
                j += 1;
            }
            word = u32::from_be(word);
            digest.0[i] = word;
            i += 1;
        }
        digest
    }
}

impl AsRef<[u8]> for Digest {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Display for Digest {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        f.write_str(&hex::encode(self))
    }
}

impl From<[u32; 8]> for Digest {
    fn from(data: [u32; 8]) -> Self {
        Self(data)
    }
}

impl From<[u8; 32]> for Digest {
    fn from(data: [u8; 32]) -> Self {
        match bytemuck::try_cast(data) {
            Ok(digest) => digest,
            Err(PodCastError::TargetAlignmentGreaterAndInputNotAligned) => {
                // Bytes are not aligned. Copy the byte array into a new digest.
                bytemuck::pod_read_unaligned(&data)
            }
            Err(e) => unreachable!("failed to cast [u8; DIGEST_BYTES] to Digest: {}", e),
        }
    }
}

#[cfg(feature = "risc0")]
impl From<risc0_zkvm::Digest> for Digest {
    fn from(digest: risc0_zkvm::Digest) -> Self {
        Self(digest.into())
    }
}

#[cfg(feature = "risc0")]
impl From<Digest> for risc0_zkvm::Digest {
    fn from(digest: Digest) -> Self {
        Self::new(digest.0)
    }
}
