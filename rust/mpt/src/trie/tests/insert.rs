use crate::{node::Node, MerkleTrie};

#[test]
pub fn empty_key() {
    let mut mpt = MerkleTrie(Node::Null);
    mpt.insert([], [42]);
    assert_eq!(mpt.get([]), Some(&[42][..]));
}

#[test]
pub fn one_byte_key() {
    let mut mpt = MerkleTrie(Node::Null);
    mpt.insert([0x0], [42]);
    assert_eq!(mpt.get([0x0]), Some(&[42][..]));
}

#[test]
pub fn multi_byte_key() {
    let mut mpt = MerkleTrie(Node::Null);
    mpt.insert([0x1, 0x1], [42]);
    assert_eq!(mpt.get([0x1, 0x1]), Some(&[42][..]));
}

#[test]
#[should_panic(expected = "DuplicatedKey(\"\\0\\0\")")]
pub fn duplicate_key() {
    let mut mpt = MerkleTrie(Node::Null);
    mpt.insert([0], [42]);
    mpt.insert([0], [43]);
}

#[test]
pub fn different_length_nibbles() {
    let mut mpt = MerkleTrie(Node::Null);
    mpt.insert([0x0], [42]);
    mpt.insert([0x10], [43]);
    assert_eq!(mpt.get([0x0]), Some(&[42][..]));
    assert_eq!(mpt.get([0x10]), Some(&[43][..]));
}
