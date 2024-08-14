use alloy_primitives::B256;
use serde::{Deserialize, Serialize};
use std::{array::from_fn, fmt::Debug};

use crate::key_nibbles::KeyNibbles;

use self::entry::Entry;

pub mod encode;
pub mod entry;

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
    #[allow(unused)]
    pub(crate) fn insert(&self, key_nibs: &[u8], value: impl AsRef<[u8]>) -> Node {
        match self {
            Node::Null => {
                if key_nibs.is_empty() {
                    Node::Branch(Default::default(), Some(value.as_ref().into()))
                } else {
                    Node::leaf(key_nibs, value)
                }
            }
            _ => Node::Null,
        }
    }
    /// Returns a reference to the value corresponding to the key.
    /// It panics when neither inclusion nor exclusion of the key can be shown in the sparse trie.
    pub(crate) fn get(&self, key_nibs: &[u8]) -> Option<&[u8]> {
        match self {
            Node::Null => None,
            Node::Leaf(prefix, value) if prefix == key_nibs => Some(value),
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

    /// Returns the number of full nodes in the trie.
    /// A full node is a node that needs to be fully encoded to compute the root hash.
    pub(crate) fn size(&self) -> usize {
        match self {
            Node::Null | Node::Digest(_) => 0,
            Node::Leaf(..) => 1,
            Node::Extension(_, child) => 1 + child.size(),
            Node::Branch(children, _) => {
                1 + children
                    .iter()
                    .filter_map(Option::as_deref)
                    .map(Node::size)
                    .sum::<usize>()
            }
        }
    }

    pub fn leaf(key_nibs: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> Node {
        Node::Leaf(key_nibs.into(), value.as_ref().into())
    }
}

impl From<Entry> for Node {
    fn from(entry: Entry) -> Self {
        if entry.key.is_empty() {
            let children = from_fn(|_| None);
            Node::Branch(children, Some(entry.value))
        } else {
            Node::leaf(entry.key, entry.value)
        }
    }
}
#[cfg(test)]
mod insert {
    use super::Node;

    #[test]
    fn empty_key() {
        let node = Node::Null.insert(&[], [42]);
        assert_eq!(Node::Branch(Default::default(), Some([42].into())), node);
    }

    #[test]
    fn short_key() {
        let node = Node::Null.insert(&[2], [42]);
        assert_eq!(Node::Leaf([2].into(), Box::new([42])), node,);
    }

    #[test]
    fn long_key() {
        let node = Node::Null.insert(&[0xF, 0xF, 0xF, 0xF], [42]);
        assert_eq!(
            Node::Leaf([0xF, 0xF, 0xF, 0xF].into(), Box::new([42])),
            node
        );
    }
}

#[cfg(test)]
mod from {
    use crate::node::{entry::Entry, Node};

    #[test]
    fn empty_key() {
        let entry = Entry::from(([], [42]));
        let node = Node::from(entry);
        assert_eq!(node, Node::Branch(Default::default(), Some([42].into())));
    }

    #[test]
    fn short_key() {
        let entry = Entry::from(([0x1], [42]));
        let node = Node::from(entry);
        assert_eq!(node, Node::Leaf([0x1].into(), Box::new([42])));
    }

    #[test]
    fn long_key() {
        let key: [u8; 4] = [0xF, 0xF, 0xF, 0xF];
        let value: [u8; 7] = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFE];
        let entry = Entry::from((key, value));
        let node = Node::from(entry);
        assert_eq!(node, Node::Leaf(key.into(), Box::new(value)));
    }
}

#[cfg(test)]
mod node_size {
    use super::Node;
    use crate::key_nibbles::KeyNibbles;
    use std::array::from_fn;

    #[test]
    fn null() {
        let node = Node::Null;
        assert_eq!(node.size(), 0);
    }

    #[test]
    fn digest() {
        let node = Node::Digest(Default::default());
        assert_eq!(node.size(), 0);
    }

    #[test]
    fn leaf() {
        let node = Node::Leaf([0x1].into(), Box::new([]));
        assert_eq!(node.size(), 1);
    }

    #[test]
    fn extension() {
        let key_nibbles: KeyNibbles = [0x1].into();
        let leaf = Node::Leaf(key_nibbles.clone(), Box::new([]));
        let extension = Node::Extension(key_nibbles, Box::new(leaf));
        assert_eq!(extension.size(), 2);
    }

    #[test]
    fn branch_one_child() {
        let leaf = Node::Leaf([0x1].into(), Box::new([]));
        let child = Some(Box::new(leaf));
        const NULL_CHILD: Option<Box<Node>> = None;
        let mut children = [NULL_CHILD; 16];
        children[0] = child;
        let branch = Node::Branch(children, None);
        assert_eq!(branch.size(), 2);
    }

    #[test]
    fn branch_many_children() {
        let leaf = Node::Leaf([0x1].into(), Box::new([]));
        let child = Some(Box::new(leaf));
        let children: [_; 16] = from_fn(|_| child.clone());
        let branch = Node::Branch(children, None);
        assert_eq!(branch.size(), 17);
    }

    #[test]
    fn branch_with_value() {
        let leaf = Node::Leaf([0x1].into(), Box::new([]));
        let child = Some(Box::new(leaf));
        const NULL_CHILD: Option<Box<Node>> = None;
        let mut children = [NULL_CHILD; 16];
        children[0] = child;
        let value = Some([42u8].as_slice().into());
        let branch = Node::Branch(children, value);
        assert_eq!(branch.get(&[]), Some(&[42u8][..]));
    }
}
