use crate::{node::Node, MerkleTrie};
use alloy_primitives::{b256, B256};
use alloy_trie::EMPTY_ROOT_HASH;
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
    let mpt = setup_branch_test(None);
    assert_eq!(
        mpt.hash_slow(),
        b256!("f09860d0bbaa3a755a53bbeb7b06824cdda5ac2ee5557d14aa49117a47bd0a3e")
    );
}

#[test]
fn branch_with_value() {
    let value = Some(vec![42u8].into());
    let mpt = setup_branch_test(value);
    assert_eq!(
        mpt.hash_slow(),
        b256!("d8234b21207e7321dbc42e0fe8688913489ad1d365b690a23508e24104e33337")
    );
}

fn setup_branch_test(value: Option<Box<[u8]>>) -> MerkleTrie {
    let mut children: [Option<Box<Node>>; 16] = Default::default();
    children[0] = Some(Box::new(Node::Leaf(
        Nibbles::from_nibbles([0; 63]),
        vec![0].into(),
    )));
    children[1] = Some(Box::new(Node::Leaf(
        Nibbles::from_nibbles([1; 63]),
        vec![1].into(),
    )));
    let mpt = MerkleTrie(Node::Branch(children, value));
    mpt
}
