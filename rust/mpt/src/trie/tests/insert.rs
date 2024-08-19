use crate::{node::Node, MerkleTrie};

#[test]
pub fn empty_key() {
    let mut mpt = MerkleTrie(Node::Null);
    mpt.insert([], [42]);
    assert_eq!(Some(&[42][..]), mpt.get([]));
}

#[test]
pub fn odd_key() {
    let mut mpt = MerkleTrie(Node::Null);
    mpt.insert([0x1], [42]);
    assert_eq!(Some(&[42][..]), mpt.get([0x01]));
}

#[test]
pub fn even_key() {
    let mut mpt = MerkleTrie(Node::Null);
    mpt.insert([0x1, 0x1], [42]);
    assert_eq!(Some(&[42][..]), mpt.get([0x1, 0x1]));
}

#[test]
#[should_panic(expected = "Key already exists")]
pub fn twice_same_key() {
    let mut mpt = MerkleTrie(Node::Null);
    mpt.insert([0], [42]);
    mpt.insert([0], [43]);
}

#[test]
pub fn two_keys() {
    let mut mpt = MerkleTrie(Node::Null);
    mpt.insert([0x1], [42]);
    mpt.insert([0x10], [43]);
    assert_eq!(Some(&[42][..]), mpt.get([0x01]));
    assert_eq!(Some(&[43][..]), mpt.get([0x10]));
}
