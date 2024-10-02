use alloy_primitives::B256;

use crate::{
    key_nibbles::KeyNibbles,
    node::{constructors::EMPTY_CHILDREN, Node},
    MerkleTrie,
};

#[test]
fn null() {
    let mpt = MerkleTrie(Node::Null);
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
fn branch_empty() {
    let mpt = MerkleTrie(Node::Branch(EMPTY_CHILDREN.clone(), None));
    let proof = mpt.to_rlp_nodes();

    let decoded_mpt = MerkleTrie::from_rlp_nodes(proof).unwrap();
    assert_eq!(mpt, decoded_mpt);
}

#[test]
fn branch_with_value() {
    let mpt = MerkleTrie(Node::branch_with_value([42]));
    let proof = mpt.to_rlp_nodes();

    let decoded_mpt = MerkleTrie::from_rlp_nodes(proof).unwrap();
    assert_eq!(mpt, decoded_mpt);
}
