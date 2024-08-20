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
pub fn two_keys_no_common_prefix() {
    let mut mpt = MerkleTrie(Node::Null);
    mpt.insert([0x02], [42]);
    mpt.insert([0x10], [43]);
    assert_eq!(Some(&[42][..]), mpt.get([0x02]));
    assert_eq!(Some(&[43][..]), mpt.get([0x10]));
}

#[test]
pub fn two_long_keys_no_common_prefix() {
    let mut mpt = MerkleTrie(Node::Null);
    mpt.insert([0x12, 0x01], [42]);
    mpt.insert([0x20, 0x01], [43]);
    assert_eq!(Some(&[42][..]), mpt.get([0x12, 0x01]));
    assert_eq!(Some(&[43][..]), mpt.get([0x20, 0x01]));
}

#[test]
pub fn one_value_empty() {
    let mut mpt = MerkleTrie(Node::Null);
    mpt.insert([0x01], [42]);
    mpt.insert([], [43]);
    assert_eq!(Some(&[42][..]), mpt.get([0x01]));
    assert_eq!(Some(&[43][..]), mpt.get([]));
}

#[test]
//todo - handle this case starting with branch
#[ignore]
pub fn one_value_empty_symmetrical() {
    let mut mpt = MerkleTrie(Node::Null);
    mpt.insert([0x01], [42]);
    mpt.insert([], [43]);
    assert_eq!(Some(&[42][..]), mpt.get([]));
    assert_eq!(Some(&[43][..]), mpt.get([0x01]));
}

#[test]
pub fn one_empty_key_one_long_key() {
    let mut mpt = MerkleTrie(Node::Null);
    mpt.insert([0xff, 0xff, 0xff, 0xff], [42]);
    mpt.insert([], [43]);
    assert_eq!(Some(&[42][..]), mpt.get([0xff, 0xff, 0xff, 0xff]));
    assert_eq!(Some(&[43][..]), mpt.get([]));
}

#[test]
pub fn common_prefix() {
    let mut mpt = MerkleTrie(Node::Null);
    mpt.insert([0xff, 0xff, 0x01], [42]);
    mpt.insert([0xff, 0xff, 0x02], [43]);
    assert_eq!(Some(&[42][..]), mpt.get([0xff, 0xff, 0x01]));
    assert_eq!(Some(&[43][..]), mpt.get([0xff, 0xff, 0x02]));
}

// #[test]
// pub fn contained_prefix() {
//     let mut mpt = MerkleTrie(Node::Null);
//     mpt.insert([0xff, 0xff, 0x01], [42]);
//     mpt.insert([0xff, 0xff], [43]);
//     assert_eq!(Some(&[42][..]), mpt.get([0xff, 0xff, 0x01]));
//     assert_eq!(Some(&[43][..]), mpt.get([0xff, 0xff, 0x02]));
// }
