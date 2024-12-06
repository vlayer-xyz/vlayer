use std::hash::Hash;

use alloy_primitives::B256;
use alloy_rlp::Decodable;
use bytes::Bytes;
use derive_more::derive::AsRef;
use mpt::Sha2Node as Node;
use serde::{Deserialize, Serialize};

use crate::ChainDbResult;

/// Node retrieved from DB. RLP representation and hash included to avoid re-calculation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, AsRef)]
pub struct DbNode {
    pub hash: Option<B256>, // None for inline nodes
    pub node: Node,
    pub rlp: Bytes,
}

impl Hash for DbNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.rlp.hash(state);
    }
}

impl DbNode {
    pub fn decode(hash: impl Into<Option<B256>>, rlp: impl Into<Bytes>) -> ChainDbResult<Self> {
        let hash = hash.into();
        let rlp = rlp.into();
        let node = Node::decode(&mut rlp.as_ref())?;
        Ok(DbNode { hash, node, rlp })
    }
}
