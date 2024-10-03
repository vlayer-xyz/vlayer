use alloy_primitives::{b256, B256};
use alloy_trie::EMPTY_ROOT_HASH;
use lazy_static::lazy_static;

use crate::{key_nibbles::KeyNibbles, node::Node, MerkleTrie};

lazy_static! {
    static ref MPT_BRANCH_WITH_TWO_CHILDREN: MerkleTrie = {
        let branch = Node::branch_with_two_children(
            0,
            Node::leaf([0; 63], [0]),
            1,
            Node::leaf([1; 63], [1]),
        );
        MerkleTrie(branch)
    };
}

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
    let mpt = MerkleTrie(Node::Leaf(KeyNibbles::unpack(B256::ZERO), [0].into()));
    assert_eq!(
        mpt.hash_slow(),
        b256!("ebcd1aff3f48f44a89c8bceb54a7e73c44edda96852b9debc4447b5ac9be19a6")
    );
}

#[test]
fn extension() {
    let leaf = Node::Leaf(KeyNibbles::unpack([1]), [0].into());
    let mpt = MerkleTrie(Node::Extension([0; 1].into(), leaf.into()));
    assert_eq!(
        mpt.hash_slow(),
        b256!("36a045336263723ea10ae76482d278f4212fe858a6937204ecc747921e2bc8c1")
    );
}

#[test]
fn branch() {
    let mpt = &MPT_BRANCH_WITH_TWO_CHILDREN;
    assert_eq!(
        mpt.hash_slow(),
        b256!("f09860d0bbaa3a755a53bbeb7b06824cdda5ac2ee5557d14aa49117a47bd0a3e")
    );
}

#[test]
fn branch_with_value() {
    let mut mpt = MPT_BRANCH_WITH_TWO_CHILDREN.clone();

    let MerkleTrie(Node::Branch(_, ref mut branch_value)) = mpt else {
        panic!("Expected a Branch node");
    };
    *branch_value = Some([42u8].into());

    assert_eq!(
        mpt.hash_slow(),
        b256!("81b2bafe48b11cf92d7b5d765155e3cab7b87f7e0e2fa8181c3535552cdafc40")
    );
}
