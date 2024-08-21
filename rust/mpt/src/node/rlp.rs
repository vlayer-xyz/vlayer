use alloy_primitives::B256;
use alloy_rlp::{Decodable, Encodable, Header, EMPTY_STRING_CODE};
use rlp as legacy_rlp;

use crate::{
    node::constructors::EMPTY_CHILDREN,
    node_ref::NodeRef,
    path::{Path, PathKind},
};

use super::Node;

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
                let mut children = EMPTY_CHILDREN.clone();
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

impl Node {
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
