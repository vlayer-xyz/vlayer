use alloy_primitives::B256;
use alloy_rlp::{Decodable, Encodable, Header, EMPTY_STRING_CODE};
use rlp as legacy_rlp;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use crate::{
    key_nibbles::KeyNibbles,
    path::{Path, PathKind},
};

use super::node_ref::NodeRef;

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
            Node::Branch(children, value) => {
                let mut child_refs: [NodeRef; 16] = Default::default();
                let mut payload_length = 0;

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

                payload_length += value
                    .as_ref()
                    .map_or(1 /* EMPTY_STRING_CODE */, |v| v.len());

                let mut out = encoded_header(true, payload_length);
                child_refs.iter().for_each(|child| child.encode(&mut out));

                out.extend_from_slice(value.as_deref().unwrap_or(&[EMPTY_STRING_CODE]));

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
                        Ok(Node::Leaf(nibbles, val.into()))
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
                let val = if !rlp.at(16)?.is_empty() {
                    Some(rlp.val_at::<Vec<u8>>(16)?.into())
                } else {
                    None
                };

                Ok(Node::Branch(children, val))
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
