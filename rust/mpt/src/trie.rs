use crate::node::Node;
use alloy_primitives::{keccak256, B256};
use alloy_rlp::Decodable;
use alloy_trie::EMPTY_ROOT_HASH;
use nybbles::Nibbles;
use rlp as legacy_rlp;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};
use thiserror::Error;
use utils::{parse_node, resolve_trie};

mod utils;

/// The error type that is returned when parsing a [MerkleTrie] node.
#[derive(Debug, Error)]
pub enum ParseNodeError {
    /// Error that occurs when parsing the RLP encoding of a node.
    #[error("RLP error")]
    Rlp(#[from] legacy_rlp::DecoderError),
}

/// A sparse Merkle Patricia trie storing byte values.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MerkleTrie(pub(crate) Node);

impl MerkleTrie {
    /// Returns a reference to the byte value corresponding to the key.
    ///
    /// It panics when neither inclusion nor exclusion of the key can be guaranteed.
    pub fn get(&self, key: impl AsRef<[u8]>) -> Option<&[u8]> {
        let nibbles = Nibbles::unpack(key);
        self.0.get(&*nibbles)
    }

    /// Inserts a key-value pair into the trie.
    ///
    /// It panics when the key already exists in the trie.
    #[cfg(test)]
    pub fn insert(&mut self, key: impl AsRef<[u8]>, value: impl AsRef<[u8]>) {
        self.0 = self.0.clone().insert(Nibbles::unpack(key), value);
    }

    /// Returns the RLP decoded value corresponding to the key.
    ///
    /// It panics when neither inclusion nor exclusion of the key can be guaranteed or when the
    /// value is not RLP decodable.
    pub fn get_rlp<T: Decodable>(&self, key: impl AsRef<[u8]>) -> alloy_rlp::Result<Option<T>> {
        match self.get(key) {
            Some(mut bytes) => Ok(Some(T::decode(&mut bytes)?)),
            None => Ok(None),
        }
    }

    /// Returns the number of full nodes in the trie.
    ///
    /// A full node is a node that needs to be fully encoded to compute the root hash.
    pub fn size(&self) -> usize {
        self.0.size()
    }

    /// Returns the hash of the trie's root node.
    pub fn hash_slow(&self) -> B256 {
        // compute the keccak hash of the RLP encoded root node
        match self.0 {
            Node::Null => EMPTY_ROOT_HASH,
            Node::Digest(digest) => digest,
            ref node => keccak256(node.rlp_encoded()),
        }
    }

    /// Creates a new trie from the given RLP encoded nodes.
    ///
    /// The first node provided must always be the root node. The remaining nodes can be in any
    /// order and are resolved if they are referenced (directly or indirectly) by the root node.
    /// Referenced children that cannot be resolved are represented by their hash. This guarantees
    /// that the root hash can be computed and matches the root hash of the fully resolved trie.
    pub fn from_rlp_nodes<T: AsRef<[u8]>>(
        nodes: impl IntoIterator<Item = T>,
    ) -> Result<Self, ParseNodeError> {
        let mut nodes_by_hash = HashMap::new();
        let mut root_node_opt = None;

        for rlp in nodes {
            let (hash, node) = parse_node(rlp)?;

            // initialize with the first node if it hasn't been set
            root_node_opt.get_or_insert(node.clone());

            if let Some(hash) = hash {
                nodes_by_hash.insert(hash, node);
            }
        }

        let root_node = root_node_opt.unwrap_or_default();
        let trie = MerkleTrie(resolve_trie(root_node.clone(), &nodes_by_hash));
        // Optional: Verify the resolved trie's hash matches the initial root's hash
        debug_assert!(trie.hash_slow() == MerkleTrie(root_node).hash_slow());

        Ok(trie)
    }
}

#[cfg(test)]
mod tests;
