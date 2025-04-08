use std::marker::PhantomData;

use alloy_primitives::B256;
use alloy_rlp::{BufMut, EMPTY_STRING_CODE, Encodable};
use bytes::Bytes;
use derivative::Derivative;

use super::node::Node;
use crate::{Digest, Keccak256, hash};

/// Represents the way in which a node is referenced from within another node.
#[derive(Debug, Default, Derivative)]
#[derivative(Clone(bound = ""))]
pub enum NodeRef<D> {
    #[default]
    Empty,
    Digest(B256),
    InlineNode(Bytes),
    Node(Bytes),
    _Phantom(PhantomData<D>),
}

impl<D: Digest> NodeRef<D> {
    pub fn from_node(node: &Node<D>) -> NodeRef<D> {
        match node {
            Node::Null => NodeRef::Empty,
            Node::Digest(digest) => NodeRef::Digest(*digest),
            node => {
                let encoded = node.rlp_encoded();
                if encoded.len() < B256::len_bytes() {
                    NodeRef::InlineNode(encoded)
                } else {
                    NodeRef::Node(encoded)
                }
            }
        }
    }

    pub fn length(&self) -> usize {
        // hash length + 1 byte for the RLP header
        const DIGEST_LENGTH: usize = 1 + B256::len_bytes();

        match self {
            NodeRef::Empty => 1,
            NodeRef::Digest(_) | NodeRef::Node(_) => DIGEST_LENGTH,
            NodeRef::InlineNode(rlp) => rlp.len(),
            NodeRef::_Phantom(_) => unreachable!(),
        }
    }
}

impl<D> Encodable for NodeRef<D>
where
    D: Digest,
{
    fn encode(&self, out: &mut dyn BufMut) {
        match self {
            NodeRef::Empty => out.put_u8(EMPTY_STRING_CODE),
            NodeRef::Digest(digest) => digest.encode(out),
            NodeRef::InlineNode(data) => out.put_slice(data),
            NodeRef::Node(rlp) => hash::<D>(rlp).encode(out),
            NodeRef::_Phantom(_) => unreachable!(),
        }
    }
}

pub type KeccakNodeRef = NodeRef<Keccak256>;
pub type Sha2NodeRef = NodeRef<sha2::Sha256>;

#[cfg(test)]
mod encodable {
    use alloy_rlp::encode;

    use super::{KeccakNodeRef as NodeRef, *};
    use crate::keccak256 as hash;

    #[test]
    fn empty() {
        let mut out = Vec::new();
        let node = NodeRef::Empty;
        node.encode(&mut out);

        assert_eq!(node.length(), 1);
        assert_eq!(out, [EMPTY_STRING_CODE]);
    }

    #[test]
    fn digest() {
        let digest = B256::repeat_byte(0x1);
        let mut out = Vec::new();
        let node = NodeRef::Digest(digest);
        node.encode(&mut out);

        assert_eq!(node.length(), 33);
        assert_eq!(out[0], EMPTY_STRING_CODE + 32);
        assert_eq!(out[1..], [0x1; 32]);
    }

    #[test]
    fn inline_node() {
        let leaf_node = Node::leaf([0x0], [0x0]);
        let node_ref = NodeRef::from_node(&leaf_node);
        let out = encode(node_ref.clone());

        let expected_rlp_encoded = leaf_node.rlp_encoded();

        assert_eq!(node_ref.length(), expected_rlp_encoded.len());
        assert_eq!(out, expected_rlp_encoded);
    }

    #[test]
    fn hash_node() {
        let rlp = Bytes::from_static(&[0; 32]);
        let hash = hash(&rlp);
        let node = NodeRef::Node(rlp);
        let out = alloy_rlp::encode(&node);

        assert_eq!(node.length(), 33);
        assert_eq!(out[0], EMPTY_STRING_CODE + 32);
        assert_eq!(&out[1..], hash.as_slice());
    }
}
