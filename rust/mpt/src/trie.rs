use std::fmt::Debug;

use alloy_primitives::B256;
use alloy_rlp::Decodable;
use bytes::Bytes;
use common::Hashable;
use derivative::Derivative;
use nybbles::Nibbles;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utils::{parse_node, resolve_trie};

use crate::{
    Digest, Keccak256,
    node::{Node, NodeError},
};

mod utils;

/// The error type that is returned when parsing a [MerkleTrie] node.
#[derive(Debug, Error)]
pub enum ParseNodeError {
    /// Error that occurs when parsing the RLP encoding of a node.
    #[error("RLP error: {0}")]
    Rlp(#[from] alloy_rlp::Error),
}

/// A sparse Merkle Patricia trie storing byte values.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Derivative)]
#[derivative(PartialEq(bound = ""), Eq(bound = ""))]
#[serde(bound = "")]
pub struct MerkleTrie<D>(pub Node<D>);

impl<D> MerkleTrie<D>
where
    D: Digest,
{
    /// Creates a new empty trie.
    pub const fn new() -> Self {
        MerkleTrie(Node::null())
    }

    /// Returns a reference to the byte value corresponding to the key.
    ///
    /// It panics when neither inclusion nor exclusion of the key can be guaranteed.
    pub fn get(&self, key: impl AsRef<[u8]>) -> Option<&[u8]> {
        self.0.get(Nibbles::unpack(key).as_slice())
    }

    /// Inserts a key-value pair into the trie.
    pub fn insert(
        &mut self,
        key: impl AsRef<[u8]>,
        value: impl AsRef<[u8]>,
    ) -> Result<(), MptError> {
        if value.as_ref().is_empty() {
            return Err(MptError::EmptyValue);
        }
        let key = key.as_ref();
        let nibbles = &*Nibbles::unpack(key);
        match self.0.clone().insert(nibbles, value) {
            Ok(new_node) => {
                self.0 = new_node;
                Ok(())
            }
            Err(NodeError::DuplicateKey) => Err(MptError::DuplicateKey(Box::from(key))),
        }
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

    /// Creates a new trie from the given RLP encoded nodes.
    ///
    /// The first node provided must always be the root node. The remaining nodes can be in any
    /// order and are resolved if they are referenced (directly or indirectly) by the root node.
    /// Referenced children that cannot be resolved are represented by their hash. This guarantees
    /// that the root hash can be computed and matches the root hash of the fully resolved trie.
    pub fn from_rlp_nodes<T: AsRef<[u8]>>(
        nodes: impl IntoIterator<Item = T>,
    ) -> Result<Self, ParseNodeError> {
        nodes.into_iter().map(parse_node::<D>).collect()
    }
}

impl<D> Hashable for MerkleTrie<D>
where
    D: Digest,
{
    /// Returns the hash of the trie's root node.
    fn hash_slow(&self) -> B256 {
        self.0.hash_slow()
    }
}

impl<D> IntoIterator for &MerkleTrie<D>
where
    D: Digest,
{
    type IntoIter = std::vec::IntoIter<Bytes>;
    type Item = Bytes;

    fn into_iter(self) -> Self::IntoIter {
        self.0.to_rlp_nodes().into_iter()
    }
}

impl<D> FromIterator<(Option<B256>, Node<D>)> for MerkleTrie<D>
where
    D: Digest,
{
    fn from_iter<T: IntoIterator<Item = (Option<B256>, Node<D>)>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let (_, root_node) = iter.next().unwrap_or_default();
        let nodes_by_hash = iter
            .filter_map(|(hash, node)| hash.map(|hash| (hash, node)))
            .collect();

        let trie = MerkleTrie(resolve_trie(root_node.clone(), &nodes_by_hash));
        // Optional: Verify the resolved trie's hash matches the initial root's hash
        debug_assert!(trie.hash_slow() == MerkleTrie(root_node).hash_slow());

        trie
    }
}

impl<K, V, D> FromIterator<(K, V)> for MerkleTrie<D>
where
    K: AsRef<[u8]>,
    V: AsRef<[u8]>,
    D: Digest,
{
    #[allow(clippy::expect_used)]
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut trie = MerkleTrie::<D>::new();
        for (key, value) in iter {
            trie.insert(key, value).expect("Insert failed");
        }
        trie
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum MptError {
    #[error("Duplicate key: {0:?}")]
    DuplicateKey(Box<[u8]>),
    #[error("Cannot insert empty value")]
    EmptyValue,
}

pub type KeccakMerkleTrie = MerkleTrie<Keccak256>;
#[allow(non_snake_case)]
pub const fn KeccakMerkleTrie(node: Node<Keccak256>) -> KeccakMerkleTrie {
    MerkleTrie::<Keccak256>(node)
}

pub type Sha2Trie = MerkleTrie<sha2::Sha256>;
#[allow(non_snake_case)]
pub const fn Sha2Trie(node: Node<sha2::Sha256>) -> Sha2Trie {
    MerkleTrie::<sha2::Sha256>(node)
}

#[cfg(test)]
mod tests;
