use alloy_primitives::B256;
use alloy_rlp::{BufMut, Decodable, Encodable, Header};
use bytes::Bytes;
use rlp as legacy_rlp;

use super::Node;
use crate::{
    Digest,
    node::constructors::empty_children,
    node_ref::NodeRef,
    path::{Path, PathKind},
};

impl<D: Digest> Encodable for Node<D> {
    fn length(&self) -> usize {
        // Default alloy implementation uses `encode`` method and then discards it's result resulting in `2**N`` time complexity.
        // This value is used to preallocate vector size.
        // Ideally - library interface should be changed. It's already in progress: https://github.com/alloy-rs/rlp/pull/29
        0
    }

    #[allow(clippy::panic)]
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
                let node_ref = NodeRef::<D>::from_node(child);
                encode_header(true, path.length() + node_ref.length(), out);
                path.encode(out);
                node_ref.encode(out);
            }
            Node::Branch(children, value) => {
                let mut child_refs: [NodeRef<D>; 16] = Default::default();
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

                let value = match value.as_deref() {
                    Some(val) if val.is_empty() => panic!("empty values are not allowed in MPT"),
                    Some(val) => val.as_ref(),
                    None => &[],
                };
                payload_length += value.length();

                encode_header(true, payload_length, out);
                child_refs.iter().for_each(|child| child.encode(out));

                value.encode(out);
            }
            Node::Digest(digest) => digest.encode(out),
            Node::_Phantom(_) => unreachable!(),
        }
    }
}

impl<D> Decodable for Node<D> {
    fn decode(buf: &mut &[u8]) -> alloy_rlp::Result<Self> {
        let rlp = legacy_rlp::Rlp::new(buf);
        legacy_rlp::Decodable::decode(&rlp).map_err(map_rlp_error)
    }
}

#[allow(clippy::needless_pass_by_value)]
const fn map_rlp_error(err: legacy_rlp::DecoderError) -> alloy_rlp::Error {
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
        rlp::DecoderError::Custom(str) => alloy_rlp::Error::Custom(str),
    }
}

// TODO: Remove `legacy_rlp` dependency and rewrite this with `alloy_rlp`
impl<D> legacy_rlp::Decodable for Node<D> {
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
                        if node == Node::<D>::Null {
                            return Err(DecoderError::Custom("extension node with null child"));
                        }
                        Ok(Node::Extension(nibbles, Box::new(node)))
                    }
                }
            }
            Prototype::List(17) => {
                let mut children = empty_children();
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

impl<D: Digest> Node<D> {
    /// Returns the RLP encoding of the node.
    pub fn rlp_encoded(&self) -> Bytes {
        alloy_rlp::encode(self).into()
    }

    pub(crate) fn to_rlp_nodes(&self) -> Vec<Bytes> {
        if matches!(self, Node::Digest(..)) {
            return vec![];
        }
        let mut nodes = vec![self.rlp_encoded()];
        let mut children = match self {
            Node::Branch(children, _) => children
                .iter()
                .flatten()
                .flat_map(|child| child.to_rlp_nodes())
                .collect(),
            Node::Extension(_, child) => child.to_rlp_nodes(),
            Node::Null | Node::Leaf(..) | Node::Digest(..) => vec![],
            Node::_Phantom(_) => unreachable!(),
        };
        nodes.append(&mut children);
        nodes
    }
}

fn encode_header(list: bool, payload_length: usize, out: &mut dyn BufMut) {
    debug_assert!(payload_length > 0);
    let header = Header {
        list,
        payload_length,
    };
    header.encode(out);
}
