use crate::{key_nibbles::KeyNibbles, node::Node, MerkleTrie};
use alloy_primitives::B256;

#[test]
fn null() {
    let mpt = MerkleTrie(Node::Null);
    let proof = mpt.to_rlp_nodes();
    assert_eq!(mpt, MerkleTrie::from_rlp_nodes(proof).unwrap());
}

#[test]
fn digest() {
    let mpt = MerkleTrie(Node::Digest(B256::ZERO));
    let proof = mpt.to_rlp_nodes();
    assert_eq!(mpt, MerkleTrie::from_rlp_nodes(proof).unwrap());
}

#[test]
fn leaf() {
    let key_nibbles: KeyNibbles = B256::ZERO.into();
    let mpt = MerkleTrie(Node::Leaf(key_nibbles, [0].into()));
    let proof = mpt.to_rlp_nodes();
    assert_eq!(mpt, MerkleTrie::from_rlp_nodes(proof).unwrap());
}

#[test]
fn branch() {
    let mpt = MerkleTrie(Node::branch_with_value([42]));
    let proof = mpt.to_rlp_nodes();

    let decoded_mpt = MerkleTrie::from_rlp_nodes(proof).unwrap();
    assert_eq!(mpt, decoded_mpt);
}
