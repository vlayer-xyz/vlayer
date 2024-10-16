use alloy_primitives::bytes::Bytes;
use mpt::MerkleTrie;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ChainProof {
    pub proof: Bytes,
    pub mpt: MerkleTrie,
}
