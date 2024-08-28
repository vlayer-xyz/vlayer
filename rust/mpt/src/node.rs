use alloy_primitives::B256;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use crate::key_nibbles::KeyNibbles;

pub mod constructors;
pub mod insert;
pub mod rlp;
pub mod size;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum Node {
    #[default]
    Null,
    Leaf(KeyNibbles, Box<[u8]>),
    Extension(KeyNibbles, Box<Node>),
    Branch([Option<Box<Node>>; 16], Option<Box<[u8]>>),
    Digest(B256),
}

impl Node {
    /// Returns a reference to the value corresponding to the key.
    /// It panics when neither inclusion nor exclusion of the key can be shown in the sparse trie.
    pub(crate) fn get(&self, key_nibs: impl AsRef<[u8]>) -> Option<&[u8]> {
        let key_nibs = key_nibs.as_ref();
        match self {
            Node::Null => None,
            Node::Leaf(prefix, value) if *prefix == key_nibs => Some(value),
            Node::Leaf(..) => None,
            Node::Extension(prefix, child) => key_nibs
                .strip_prefix(prefix.as_slice())
                .and_then(|remaining| child.get(remaining)),
            Node::Branch(children, value) => {
                if key_nibs.is_empty() {
                    value.as_deref()
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

#[derive(Debug, PartialEq)]
pub enum MPTError {
    DuplicatedKey(String),
}
