use alloy_primitives::{b256, keccak256, B256};
use alloy_rlp::Decodable;
use node::Node;
use nybbles::Nibbles;
use revm::primitives::HashMap;
use rlp as legacy_rlp;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use thiserror::Error as ThisError;

mod node;
mod node_ref;

/// Root hash of an empty Merkle Patricia trie, i.e. `keccak256(RLP(""))`.
pub const EMPTY_ROOT_HASH: B256 =
    b256!("56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421");

/// The error type that is returned when parsing a [MerkleTrie] node.
#[derive(Debug, ThisError)]
pub enum ParseNodeError {
    /// Error that occurs when parsing the RLP encoding of a node.
    #[error("RLP error")]
    Rlp(#[from] legacy_rlp::DecoderError),
}

/// A sparse Merkle Patricia trie storing byte values.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MerkleTrie(Node);

impl MerkleTrie {
    /// Returns a reference to the byte value corresponding to the key.
    ///
    /// It panics when neither inclusion nor exclusion of the key can be guaranteed.
    #[inline]
    pub fn get(&self, key: impl AsRef<[u8]>) -> Option<&[u8]> {
        self.0.get(Nibbles::unpack(key).as_slice())
    }

    /// Returns the RLP decoded value corresponding to the key.
    ///
    /// It panics when neither inclusion nor exclusion of the key can be guaranteed or when the
    /// value is not RLP decodable.
    #[inline]
    pub fn get_rlp<T: Decodable>(&self, key: impl AsRef<[u8]>) -> alloy_rlp::Result<Option<T>> {
        match self.get(key) {
            Some(mut bytes) => Ok(Some(T::decode(&mut bytes)?)),
            None => Ok(None),
        }
    }

    /// Returns the number of full nodes in the trie.
    ///
    /// A full node is a node that needs to be fully encoded to compute the root hash.
    #[inline]
    pub fn size(&self) -> usize {
        self.0.size()
    }

    /// Returns the hash of the trie's root node.
    #[inline]
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

/// Returns the decoded node and its RLP hash.
fn parse_node(rlp: impl AsRef<[u8]>) -> Result<(Option<B256>, Node), ParseNodeError> {
    let rlp = rlp.as_ref();
    let node = legacy_rlp::decode(rlp)?;
    // the hash is only needed for RLP length >= 32
    Ok(((rlp.len() >= 32).then(|| keccak256(rlp)), node))
}

fn resolve_trie(root: Node, nodes_by_hash: &HashMap<B256, Node>) -> Node {
    match root {
        Node::Null | Node::Leaf(..) => root,
        Node::Extension(prefix, child) => {
            Node::Extension(prefix, Box::new(resolve_trie(*child, nodes_by_hash)))
        }
        Node::Branch(mut children) => {
            // iterate over the children in place, resolving each child node recursively.
            for child in children.iter_mut() {
                if let Some(node) = child.take() {
                    *child = Some(Box::new(resolve_trie(*node, nodes_by_hash)));
                }
            }
            Node::Branch(children)
        }
        Node::Digest(digest) => match nodes_by_hash.get(&digest) {
            Some(node) => resolve_trie(node.clone(), nodes_by_hash),
            None => root,
        },
    }
}

#[cfg(test)]
mod tests;
