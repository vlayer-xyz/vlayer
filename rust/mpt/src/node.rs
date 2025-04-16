use std::marker::PhantomData;

use alloy_primitives::{B256, Bytes};
use common::Hashable;
use derivative::Derivative;
use nybbles::Nibbles;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{Digest, Keccak256, hash};

pub mod constructors;
pub mod insert;
pub mod rlp;
pub mod size;

#[derive(Debug, Default, Serialize, Deserialize, Derivative)]
#[derivative(Clone(bound = ""), PartialEq(bound = ""), Eq(bound = ""))]
#[serde(bound = "")]
pub enum Node<D> {
    #[default]
    Null,
    Leaf(Nibbles, Bytes),
    Extension(Nibbles, Box<Node<D>>),
    Branch([Option<Box<Node<D>>>; 16], Option<Bytes>),
    Digest(B256),
    _Phantom(PhantomData<D>),
}

impl<D> Node<D>
where
    D: Digest,
{
    /// Returns a reference to the value corresponding to the key.
    /// It panics when neither inclusion nor exclusion of the key can be shown in the sparse trie.
    #[allow(clippy::panic)]
    pub(crate) fn get(&self, key_nibs: impl AsRef<[u8]>) -> Option<&[u8]> {
        let key_nibs = key_nibs.as_ref();
        match self {
            Node::Leaf(prefix, value) if *prefix == *key_nibs => Some(value),
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
            Node::_Phantom(_) => unreachable!(),
        }
    }
}

impl<D> Hashable for Node<D>
where
    D: Digest,
{
    fn hash_slow(&self) -> B256 {
        match self {
            Node::Null => D::EMPTY_ROOT_HASH,
            Node::Digest(digest) => *digest,
            node => hash::<D>(node.rlp_encoded()),
        }
    }
}

pub type KeccakNode = Node<Keccak256>;
pub type Sha2Node = Node<sha2::Sha256>;

#[derive(Error, Debug, PartialEq)]
pub enum NodeError {
    #[error("duplicate key")]
    DuplicateKey,
}
