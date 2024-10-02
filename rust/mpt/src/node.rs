use alloy_primitives::{Bytes, B256};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::key_nibbles::KeyNibbles;

pub mod constructors;
pub mod insert;
pub mod rlp;
pub mod size;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Node {
    #[default]
    Null,
    Leaf(KeyNibbles, Bytes),
    Extension(KeyNibbles, Box<Node>),
    Branch([Option<Box<Node>>; 16], Option<Bytes>),
    Digest(B256),
}

impl Node {
    /// Returns a reference to the value corresponding to the key.
    /// It panics when neither inclusion nor exclusion of the key can be shown in the sparse trie.
    pub(crate) fn get(&self, key_nibs: impl AsRef<[u8]>) -> Option<&[u8]> {
        let key_nibs = key_nibs.as_ref();
        match self {
            Node::Leaf(prefix, value) if *prefix == key_nibs => Some(value),
            Node::Leaf(..) | Node::Null => None,
            Node::Extension(prefix, child) => key_nibs
                .strip_prefix(prefix.as_slice())
                .and_then(|remaining| child.get(remaining)),
            Node::Branch(children, value) => {
                if key_nibs.is_empty() {
                    value.as_deref().map(AsRef::as_ref)
                } else {
                    let (idx, remaining) = key_nibs.split_first()?;
                    let child = children[*idx as usize].as_deref()?;
                    child.get(remaining)
                }
            }
            Node::Digest(_) => panic!("Attempted to access unresolved node"),
        }
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum NodeError {
    #[error("duplicate key")]
    DuplicateKey,
}
