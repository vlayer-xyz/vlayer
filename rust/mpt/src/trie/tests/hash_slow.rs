use super::*;

use alloy_primitives::{b256, B256};
use nybbles::Nibbles;

#[test]
fn null() {
    let mpt = MerkleTrie(Node::Null);
    assert_eq!(mpt.hash_slow(), EMPTY_ROOT_HASH);
}

#[test]
fn digest() {
    let mpt = MerkleTrie(Node::Digest(B256::ZERO));
    assert_eq!(mpt.hash_slow(), B256::ZERO);
}

#[test]
fn leaf() {
    let mpt = MerkleTrie(Node::Leaf(Nibbles::unpack(B256::ZERO), vec![0].into()));
    assert_eq!(
        mpt.hash_slow(),
        b256!("ebcd1aff3f48f44a89c8bceb54a7e73c44edda96852b9debc4447b5ac9be19a6")
    );
}

#[test]
fn extension() {
    let leaf = Node::Leaf(Nibbles::unpack([1]), vec![0].into());
    let mpt = MerkleTrie(Node::Extension(Nibbles::from_nibbles([0; 1]), leaf.into()));
    assert_eq!(
        mpt.hash_slow(),
        b256!("36a045336263723ea10ae76482d278f4212fe858a6937204ecc747921e2bc8c1")
    );
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
    assert_eq!(
        mpt.hash_slow(),
        b256!("f09860d0bbaa3a755a53bbeb7b06824cdda5ac2ee5557d14aa49117a47bd0a3e")
    );
}
