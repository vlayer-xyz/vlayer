use std::array::from_fn;

use alloy_primitives::{b256, B256};

use crate::{
    key_nibbles::KeyNibbles,
    node::{constructors::EMPTY_CHILDREN, Node},
    MerkleTrie,
};

#[test]
pub fn null() {
    let mpt = MerkleTrie(Node::Null);
    // the empty trie provides a non-inclusion proof for any key
    assert_eq!(mpt.get([]), None);
    assert_eq!(mpt.get([0]), None);
    assert_eq!(mpt.get([1, 2, 3]), None);
}

#[test]
#[should_panic(expected = "Attempted to access unresolved node")]
fn digest() {
    let mpt = MerkleTrie(Node::Digest(B256::ZERO));
    mpt.get([]);
}

#[test]
fn leaf() {
    let leaf_key = KeyNibbles::unpack(B256::ZERO);
    let mpt = MerkleTrie(Node::leaf(leaf_key.as_slice(), [0]));
    // A single leaf proves the inclusion of the key and non-inclusion of any other key
    assert_eq!(mpt.get(B256::ZERO).unwrap(), [0]);
    assert_eq!(mpt.get([]), None);
    assert_eq!(mpt.get([0]), None);
    assert_eq!(mpt.get([1, 2, 3]), None);
}

#[test]
fn branch() {
    let mut children = EMPTY_CHILDREN.clone();
    children[0] = Some(Box::new(Node::leaf([0; 63], [0])));
    children[1] = Some(Box::new(Node::leaf([1; 63], [1])));

    let mpt = MerkleTrie(Node::Branch(children, None));

    assert_eq!(mpt.get(B256::repeat_byte(0x00)).unwrap(), [0]);
    assert_eq!(mpt.get(B256::repeat_byte(0x11)).unwrap(), [1]);
    assert_eq!(mpt.get([]), None);
    assert_eq!(mpt.get([0]), None);
    assert_eq!(mpt.get([1, 2, 3]), None);
}

#[test]
fn branch_with_value() {
    let mpt = MerkleTrie(Node::branch_with_value(from_fn(|_| None), [42u8]));

    assert_eq!(mpt.get([]).unwrap(), [42]);
    assert_eq!(mpt.get([0]), None);
    assert_eq!(mpt.get([1, 2, 3]), None);
}

#[test]
fn extension() {
    let mut children = EMPTY_CHILDREN.clone();
    children[0] = Some(Box::new(Node::leaf([0; 62], [0])));
    children[1] = Some(Box::new(Node::leaf([1; 62], [1])));
    let branch = Node::Branch(children, None);
    let mpt = MerkleTrie(Node::extension([0], branch));

    assert_eq!(mpt.get(B256::ZERO).unwrap(), [0]);
    assert_eq!(
        mpt.get(b256!(
            "0111111111111111111111111111111111111111111111111111111111111111"
        ))
        .unwrap(),
        [1]
    );
    assert_eq!(mpt.get([]), None);
    assert_eq!(mpt.get([0]), None);
    assert_eq!(mpt.get([1, 2, 3]), None);
    assert_eq!(mpt.get(B256::repeat_byte(0x11)), None);
}
