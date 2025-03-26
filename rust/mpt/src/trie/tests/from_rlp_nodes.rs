use alloy_primitives::B256;
use nybbles::Nibbles;

use crate::{KeccakMerkleTrie as MerkleTrie, node::Node};

#[test]
fn null() {
    let mpt = MerkleTrie(Node::Null);
    assert_eq!(mpt, MerkleTrie::from_rlp_nodes(&mpt).unwrap());
}

#[test]
fn leaf() {
    let key_nibbles = Nibbles::unpack(B256::ZERO);
    let mpt = MerkleTrie(Node::Leaf(key_nibbles, [0].into()));
    assert_eq!(mpt, MerkleTrie::from_rlp_nodes(&mpt).unwrap());
}

#[test]
fn branch_empty() {
    let mpt = MerkleTrie(Node::empty_branch());

    let decoded_mpt = MerkleTrie::from_rlp_nodes(&mpt).unwrap();
    assert_eq!(mpt, decoded_mpt);
}

#[test]
fn branch_with_value() {
    let mpt = MerkleTrie(Node::branch_with_value([42]));

    let decoded_mpt = MerkleTrie::from_rlp_nodes(&mpt).unwrap();
    assert_eq!(mpt, decoded_mpt);
}

#[test]
fn nested_brach_with_multibyte_value() {
    // There was a bug in RLP encoding that occured when a branch contained a nested brach,
    // wich contained a value that serialized to more than one byte.
    let mpt =
        MerkleTrie(Node::branch_with_child_and_value(0, Node::branch_with_value([0, 0]), [0]));

    let decoded_mpt = MerkleTrie::from_rlp_nodes(&mpt).unwrap();
    assert_eq!(mpt, decoded_mpt);
}
