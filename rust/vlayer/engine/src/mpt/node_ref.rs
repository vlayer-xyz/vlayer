use alloy_primitives::{keccak256, B256};
use alloy_rlp::{BufMut, Encodable, EMPTY_STRING_CODE};

use super::node::Node;

/// Represents the way in which a node is referenced from within another node.
#[derive(Default)]
pub(crate) enum NodeRef<'a> {
    #[default]
    Empty,
    Digest(&'a B256),
    Node(Vec<u8>),
}

impl NodeRef<'_> {
    #[inline]
    pub(crate) fn from_node(node: &Node) -> NodeRef<'_> {
        match node {
            Node::Null => NodeRef::Empty,
            Node::Digest(digest) => NodeRef::Digest(digest),
            node => NodeRef::Node(node.rlp_encoded()),
        }
    }
}

impl Encodable for NodeRef<'_> {
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
        }
    }
}
