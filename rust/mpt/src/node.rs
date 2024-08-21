use alloy_primitives::B256;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use crate::key_nibbles::KeyNibbles;

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
    pub fn leaf(key_nibs: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> Node {
        Node::Leaf(key_nibs.into(), value.as_ref().into())
    }

    #[allow(unused)]
    pub fn extension(key_nibs: impl AsRef<[u8]>, value: Node) -> Node {
        Node::Extension(key_nibs.into(), value.into())
    }

    #[allow(unused)]
    pub fn branch(children: [Option<Box<Node>>; 16], value: Option<impl AsRef<[u8]>>) -> Node {
        let value = value.map(|v| v.as_ref().into());
        Node::Branch(children, value)
    }

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
