use alloy_primitives::B256;
use alloy_rlp::{Decodable, Encodable, Header, EMPTY_STRING_CODE};
use nybbles::Nibbles;
use rlp as legacy_rlp;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use crate::path::{Path, PathKind};

use super::node_ref::NodeRef;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum Node {
    #[default]
    Null,
    Leaf(Nibbles, Box<[u8]>),
    Extension(Nibbles, Box<Node>),
    Branch([Option<Box<Node>>; 16]),
    Digest(B256),
}

impl Node {
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
            Node::Branch(children) => {
                let (idx, remaining) = key_nibs.split_first()?;
                let child = children[*idx as usize].as_deref()?;
                child.get(remaining)
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
            Node::Branch(children) => {
                1 + children
                    .iter()
                    .filter_map(Option::as_deref)
                    .map(Node::size)
                    .sum::<usize>()
            }
        }
    }

    /// Returns the RLP encoding of the node.
    pub(crate) fn rlp_encoded(&self) -> Vec<u8> {
        match self {
            Node::Null => vec![EMPTY_STRING_CODE],
            Node::Leaf(prefix, value) => {
                let path = prefix.encode_path_leaf(true);
                let mut out = encoded_header(true, path.length() + value.length());
                path.encode(&mut out);
                value.encode(&mut out);

                out
            }
            Node::Extension(prefix, child) => {
                let path = prefix.encode_path_leaf(false);
                let node_ref = NodeRef::from_node(child);
                let mut out = encoded_header(true, path.length() + node_ref.length());
                path.encode(&mut out);
                node_ref.encode(&mut out);

                out
            }
            Node::Branch(children) => {
                let mut child_refs: [NodeRef; 16] = Default::default();
                let mut payload_length = 1; // start with 1 for the EMPTY_STRING_CODE at the end

                for (i, child) in children.iter().enumerate() {
                    match child.as_deref() {
                        Some(node) => {
                            let node_ref = NodeRef::from_node(node);
                            payload_length += node_ref.length();
                            child_refs[i] = node_ref;
                        }
                        None => payload_length += 1,
                    }
                }

                let mut out = encoded_header(true, payload_length);
                child_refs.iter().for_each(|child| child.encode(&mut out));
                // add an EMPTY_STRING_CODE for the missing value
                out.push(EMPTY_STRING_CODE);

                out
            }
            Node::Digest(digest) => alloy_rlp::encode(digest),
        }
    }
}

impl legacy_rlp::Decodable for Node {
    fn decode(rlp: &legacy_rlp::Rlp) -> Result<Self, legacy_rlp::DecoderError> {
        use legacy_rlp::{Decodable, DecoderError, Prototype};

        match rlp.prototype()? {
            Prototype::Null | Prototype::Data(0) => Ok(Node::Null),
            Prototype::List(2) => {
                let Path { kind, nibbles } = rlp.val_at::<Vec<u8>>(0)?.into();
                match kind {
                    PathKind::Leaf => {
                        let val = rlp.val_at::<Vec<u8>>(1)?;
                        Ok(Node::Leaf(nibbles, val.into_boxed_slice()))
                    }
                    PathKind::Extension => {
                        let node = Decodable::decode(&rlp.at(1)?)?;
                        if node == Node::Null {
                            return Err(DecoderError::Custom("extension node with null child"));
                        }
                        Ok(Node::Extension(nibbles, Box::new(node)))
                    }
                }
            }
            Prototype::List(17) => {
                let mut children: [Option<Box<Node>>; 16] = Default::default();
                for (i, node_rlp) in rlp.iter().enumerate().take(16) {
                    match node_rlp.prototype()? {
                        Prototype::Null | Prototype::Data(0) => {}
                        _ => children[i] = Some(Box::new(Decodable::decode(&node_rlp)?)),
                    }
                }
                // verify that there is no 17th element with a value
                if !rlp.at(16)?.is_empty() {
                    return Err(DecoderError::Custom("branch node with value"));
                }

                Ok(Node::Branch(children))
            }
            Prototype::Data(32) => {
                let digest = B256::decode(&mut rlp.as_raw())
                    .map_err(|_| DecoderError::Custom("invalid digest"))?;
                Ok(Node::Digest(digest))
            }
            _ => Err(DecoderError::RlpIncorrectListLen),
        }
    }
}

#[inline]
fn encoded_header(list: bool, payload_length: usize) -> Vec<u8> {
    debug_assert!(payload_length > 0);
    let header = Header {
        list,
        payload_length,
    };
    let mut out = Vec::with_capacity(header.length() + payload_length);
    header.encode(&mut out);
    out
}

#[cfg(test)]
mod node_size {
    use nybbles::Nibbles;

    use super::Node;

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
        let node = Node::Leaf(Nibbles::default(), Box::new([]));
        assert_eq!(node.size(), 1);
    }

    #[test]
    fn extension() {
        let leaf = Node::Leaf(Nibbles::default(), Box::new([]));
        let extension = Node::Extension(Nibbles::default(), Box::new(leaf));
        assert_eq!(extension.size(), 2);
    }

    #[test]
    fn branch() {
        let leaf = Node::Leaf(Nibbles::default(), Box::new([]));
        const NULL_CHILD: Option<Box<Node>> = None;
        let mut children = [NULL_CHILD; 16];
        children[0] = Some(Box::new(leaf));
        let branch = Node::Branch(children);
        assert_eq!(branch.size(), 2);
    }
}
