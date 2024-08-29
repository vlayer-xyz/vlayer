use crate::{node::Node, trie::MPTError, MerkleTrie};

#[test]
pub fn empty_key() {
    let mut mpt = MerkleTrie(Node::Null);
    mpt.insert([], [42]).unwrap();
    assert_eq!(mpt.get([]), Some(&[42][..]));
}

#[test]
pub fn one_byte_key() {
    let mut mpt = MerkleTrie(Node::Null);
    mpt.insert([0x0], [42]).unwrap();
    assert_eq!(mpt.get([0x0]), Some(&[42][..]));
}

#[test]
pub fn duplicate_key() {
    let mut mpt = MerkleTrie(Node::Null);
    mpt.insert([0], [42]).unwrap();
    let result = mpt.insert([0], [43]);
    assert!(result.is_err(), "Expected an error, but got Ok");
    assert_eq!(
        result.unwrap_err(),
        MPTError::DuplicatedKey("\0".to_string())
    );
}

#[test]
pub fn multi_byte_key() {
    let mut mpt = MerkleTrie(Node::Null);
    mpt.insert([0x1, 0x1], [42]).unwrap();
    assert_eq!(mpt.get([0x1, 0x1]), Some(&[42][..]));
}

#[test]
pub fn different_length_nibbles() {
    let mut mpt = MerkleTrie(Node::Null);
    mpt.insert([0x0], [42]).unwrap();
    mpt.insert([0x10], [43]).unwrap();
    assert_eq!(mpt.get([0x0]), Some(&[42][..]));
    assert_eq!(mpt.get([0x10]), Some(&[43][..]));
}
