use crate::{
    key_nibbles::KeyNibbles,
    node::{constructors::EMPTY_CHILDREN, Node},
    MerkleTrie,
};
use alloy_primitives::B256;

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

#[test]
fn nested_brach_with_multibyte_value() {
    // There was a bug in RLP encoding that occured when a branch contained a nested brach,
    // wich contained a value that serialized to more than one byte.
    let mpt =
        MerkleTrie(Node::branch_with_child_and_value(0, Node::branch_with_value([0, 0]), [0]));
    let proof = mpt.to_rlp_nodes();

    let decoded_mpt = MerkleTrie::from_rlp_nodes(proof).unwrap();
    assert_eq!(mpt, decoded_mpt);
}
