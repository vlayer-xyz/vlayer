use alloy_primitives::B256;
use alloy_rlp::{BufMut, Decodable, Encodable, Header};
use rlp as legacy_rlp;

use crate::{
    node::constructors::EMPTY_CHILDREN,
    node_ref::NodeRef,
    path::{Path, PathKind},
};

use super::Node;

impl Encodable for Node {
    fn encode(&self, out: &mut dyn BufMut) {
        match self {
            Node::Null => "".encode(out),
            Node::Leaf(prefix, value) => {
                let path = prefix.encode_path_leaf(true);
                encode_header(true, path.length() + value.length(), out);
                path.encode(out);
                value.encode(out);
            }
            Node::Extension(prefix, child) => {
                let path = prefix.encode_path_leaf(false);
                let node_ref = NodeRef::from_node(child);
                encode_header(true, path.length() + node_ref.length(), out);
                path.encode(out);
                node_ref.encode(out);
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

                encode_header(true, payload_length, out);
                child_refs.iter().for_each(|child| child.encode(out));

                let value = value.as_deref().unwrap_or(&[]);
                value.encode(out);
            }
            Node::Digest(digest) => digest.encode(out),
        }
    }
}

impl Decodable for Node {
    fn decode(buf: &mut &[u8]) -> alloy_rlp::Result<Self> {
        let rlp = legacy_rlp::Rlp::new(buf);
        legacy_rlp::Decodable::decode(&rlp).map_err(map_rlp_error)
    }
}

#[allow(clippy::needless_pass_by_value)]
fn map_rlp_error(err: legacy_rlp::DecoderError) -> alloy_rlp::Error {
    match err {
        rlp::DecoderError::RlpIsTooBig | rlp::DecoderError::RlpInconsistentLengthAndData => {
            alloy_rlp::Error::UnexpectedLength
        }
        rlp::DecoderError::RlpIsTooShort => alloy_rlp::Error::InputTooShort,
        rlp::DecoderError::RlpDataLenWithZeroPrefix
        | rlp::DecoderError::RlpListLenWithZeroPrefix => alloy_rlp::Error::LeadingZero,
        rlp::DecoderError::RlpInvalidLength => alloy_rlp::Error::Overflow,
        rlp::DecoderError::RlpExpectedToBeList => alloy_rlp::Error::Custom("list expected"),
        rlp::DecoderError::RlpExpectedToBeData => alloy_rlp::Error::Custom("data expected"),
        rlp::DecoderError::RlpIncorrectListLen => alloy_rlp::Error::Custom("incorrect list length"),
        rlp::DecoderError::RlpInvalidIndirection => alloy_rlp::Error::Custom("invalid indirection"),
        rlp::DecoderError::Custom(_) => alloy_rlp::Error::Custom("unknown error"), // Cannot convert &str to &'static str, co message is discarded
    }
}

// TODO: Remove `legacy_rlp` dependency and rewrite this with `alloy_rlp`
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
                let val = (!rlp.at(16)?.is_empty())
                    .then(|| rlp.val_at::<Vec<u8>>(16))
                    .transpose()?
                    .map(Into::into);

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
    pub fn rlp_encoded(&self) -> Vec<u8> {
        alloy_rlp::encode(self)
    }

    pub(crate) fn to_rlp_nodes(&self) -> Vec<Vec<u8>> {
        let mut out = vec![self.rlp_encoded()];
        match self {
            Node::Branch(children, _) => {
                for child in children.iter().flatten() {
                    out.extend(child.to_rlp_nodes());
                }
            }
            Node::Extension(_, child) => {
                out.extend(child.to_rlp_nodes());
            }
            _ => {}
        };
        out
    }
}

#[inline]
fn encode_header(list: bool, payload_length: usize, out: &mut dyn BufMut) {
    debug_assert!(payload_length > 0);
    let header = Header {
        list,
        payload_length,
    };
    header.encode(out);
}
