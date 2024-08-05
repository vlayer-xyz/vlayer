use super::*;

use alloy_primitives::{b256, B256};
use nybbles::Nibbles;

#[test]
pub fn null() {
    let mpt = MerkleTrie(Node::Null);
    // the empty trie provides a non-inclusion proof for any key
    assert_eq!(mpt.get([]), None);
    assert_eq!(mpt.get([0]), None);
    assert_eq!(mpt.get([1, 2, 3]), None);
}

#[test]
#[should_panic]
fn digest() {
    let mpt = MerkleTrie(Node::Digest(B256::ZERO));
    mpt.get([]);
}

#[test]
fn leaf() {
    let mpt = MerkleTrie(Node::Leaf(Nibbles::unpack(B256::ZERO), vec![0].into()));
    // a single leave proves the inclusion of the key and non-inclusion of any other key
    assert_eq!(mpt.get(B256::ZERO), Some(&[0][..]));
    assert_eq!(mpt.get([]), None);
    assert_eq!(mpt.get([0]), None);
    assert_eq!(mpt.get([1, 2, 3]), None);
}

#[test]
fn branch() {
    let mut children: [Option<Box<Node>>; 16] = Default::default();
    children[0] = Some(Box::new(Node::Leaf(
        Nibbles::from_nibbles([0; 63]),
        vec![0].into(),
    )));
    children[1] = Some(Box::new(Node::Leaf(
        Nibbles::from_nibbles([1; 63]),
        vec![1].into(),
    )));
    let mpt = MerkleTrie(Node::Branch(children));

    assert_eq!(mpt.get(B256::repeat_byte(0x00)), Some(&[0][..]));
    assert_eq!(mpt.get(B256::repeat_byte(0x11)), Some(&[1][..]));
    assert_eq!(mpt.get([]), None);
    assert_eq!(mpt.get([0]), None);
    assert_eq!(mpt.get([1, 2, 3]), None);
}

#[test]
fn extension() {
    let mut children: [Option<Box<Node>>; 16] = Default::default();
    children[0] = Some(Box::new(Node::Leaf(
        Nibbles::from_nibbles([0; 62]),
        vec![0].into(),
    )));
    children[1] = Some(Box::new(Node::Leaf(
        Nibbles::from_nibbles([1; 62]),
        vec![1].into(),
    )));
    let branch = Node::Branch(children);
    let mpt = MerkleTrie(Node::Extension(
        Nibbles::from_nibbles([0; 1]),
        branch.into(),
    ));

    assert_eq!(mpt.get(B256::ZERO), Some(&[0][..]));
    assert_eq!(
        mpt.get(b256!(
            "0111111111111111111111111111111111111111111111111111111111111111"
        )),
        Some(&[1][..])
    );
    assert_eq!(mpt.get([]), None);
    assert_eq!(mpt.get([0]), None);
    assert_eq!(mpt.get([1, 2, 3]), None);
    assert_eq!(mpt.get(B256::repeat_byte(0x11)), None);
}
