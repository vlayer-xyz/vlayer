use alloy_primitives::Bytes;
use serde::{Deserialize, Serialize};

mod block_storage;
mod in_memory;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ChainProof {
    pub proof: Bytes,
    pub nodes: Vec<Bytes>,
}

impl ChainProof {
    pub fn new(proof: Bytes, nodes: Vec<Bytes>) -> Self {
        ChainProof { proof, nodes }
    }
}
