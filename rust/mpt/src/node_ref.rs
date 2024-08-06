use alloy_primitives::{keccak256, B256};
use alloy_rlp::{BufMut, Encodable, EMPTY_STRING_CODE};

use super::node::Node;

/// Represents the way in which a node is referenced from within another node.
#[derive(Default)]
pub(crate) enum NodeRef {
    #[default]
    Empty,
    Digest(B256),
    Node(Vec<u8>),
    InlineNode(Vec<u8>),
}

impl NodeRef {
    #[inline]
    pub(crate) fn from_node(node: &Node) -> NodeRef {
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
}

impl Encodable for NodeRef {
    #[inline]
    fn encode(&self, out: &mut dyn BufMut) {
        match self {
            NodeRef::Empty => out.put_u8(EMPTY_STRING_CODE),
            NodeRef::Digest(digest) => digest.encode(out),
            NodeRef::Node(rlp) => {
                if rlp.len() >= B256::len_bytes() {
                    keccak256(rlp).encode(out);
                } else {
                    out.put_slice(rlp);
                }
            }
            NodeRef::InlineNode(data) => out.put_slice(data),
        }
    }

    #[inline]
    fn length(&self) -> usize {
        // hash length + 1 byte for the RLP header
        const DIGEST_LENGTH: usize = 1 + B256::len_bytes();

        match self {
            NodeRef::Empty => 1,
            NodeRef::Digest(_) => DIGEST_LENGTH,
            NodeRef::Node(rlp) => {
                if rlp.len() >= B256::len_bytes() {
                    DIGEST_LENGTH
                } else {
                    rlp.len()
                }
            }
            NodeRef::InlineNode(data) => data.len(),
        }
    }
}

#[cfg(test)]
mod encodable {
    use super::*;

    #[test]
    fn empty() {
        let mut out = Vec::new();
        let node = NodeRef::Empty;
        node.encode(&mut out);

        assert_eq!(node.length(), 1);
        assert_eq!(out, vec![EMPTY_STRING_CODE]);
    }

    #[test]
    fn digest() {
        let digest = B256::repeat_byte(0x1);
        let mut out = Vec::new();
        let node = NodeRef::Digest(digest);
        node.encode(&mut out);

        assert_eq!(node.length(), 33);
        assert_eq!(out[0], EMPTY_STRING_CODE + 32);
        assert_eq!(out[1..], vec![0x1; 32]);
    }

    #[test]
    fn inline_node() {
        let rlp = vec![0x1];
        let mut out = Vec::new();
        let node = NodeRef::Node(rlp.clone());
        node.encode(&mut out);

        assert_eq!(node.length(), 1);
        assert_eq!(out, rlp);
    }

    #[test]
    fn hash_node() {
        let rlp = vec![0x1; 32];
        let hash = keccak256(&rlp);
        let mut out = Vec::new();
        let node = NodeRef::Node(rlp.clone());
        node.encode(&mut out);

        assert_eq!(node.length(), 33);
        assert_eq!(out[0], EMPTY_STRING_CODE + 32);
        assert_eq!(&out[1..], hash.as_slice());
    }
}
