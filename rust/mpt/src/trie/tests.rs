use super::*;

use alloy_primitives::{b256, keccak256, Bytes, B256, U256};
use alloy_trie::HashBuilder;
use nybbles::Nibbles;
use std::collections::BTreeMap;

mod e2e;

/// Hash of an empty byte array, i.e. `keccak256([])`.
pub const KECCAK_EMPTY: B256 =
    b256!("c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470");

fn rlp_encoded(root: &Node) -> Vec<Vec<u8>> {
    let mut out = vec![root.rlp_encoded()];
    match root {
        Node::Null | Node::Leaf(_, _) | Node::Digest(_) => {}
        Node::Extension(_, child) => out.extend(rlp_encoded(child)),
        Node::Branch(children) => {
            out.extend(children.iter().flatten().flat_map(|c| rlp_encoded(c)));
        }
    };
    out
}

#[test]
pub fn mpt_null() {
    let mpt = MerkleTrie(Node::Null);
    assert_eq!(
        mpt,
        MerkleTrie::from_rlp_nodes(rlp_encoded(&mpt.0)).unwrap()
    );

    assert_eq!(mpt.hash_slow(), EMPTY_ROOT_HASH);

    // the empty trie provides a non-inclusion proof for any key
    assert_eq!(mpt.get([]), None);
    assert_eq!(mpt.get([0]), None);
    assert_eq!(mpt.get([1, 2, 3]), None);
}

#[test]
pub fn mpt_digest() {
    let mpt = MerkleTrie(Node::Digest(B256::ZERO));
    assert_eq!(
        mpt,
        MerkleTrie::from_rlp_nodes(rlp_encoded(&mpt.0)).unwrap()
    );

    assert_eq!(mpt.hash_slow(), B256::ZERO);
}

#[test]
pub fn mpt_leaf() {
    let mpt = MerkleTrie(Node::Leaf(Nibbles::unpack(B256::ZERO), vec![0].into()));
    assert_eq!(
        mpt,
        MerkleTrie::from_rlp_nodes(rlp_encoded(&mpt.0)).unwrap()
    );

    assert_eq!(
        mpt.hash_slow(),
        b256!("ebcd1aff3f48f44a89c8bceb54a7e73c44edda96852b9debc4447b5ac9be19a6")
    );

    // a single leave proves the inclusion of the key and non-inclusion of any other key
    assert_eq!(mpt.get(B256::ZERO), Some(&[0][..]));
    assert_eq!(mpt.get([]), None);
    assert_eq!(mpt.get([0]), None);
    assert_eq!(mpt.get([1, 2, 3]), None);
}

#[test]
pub fn mpt_branch() {
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

    assert_eq!(mpt.get(B256::repeat_byte(0x00)), Some(&[0][..]));
    assert_eq!(mpt.get(B256::repeat_byte(0x11)), Some(&[1][..]));
    assert_eq!(mpt.get([]), None);
    assert_eq!(mpt.get([0]), None);
    assert_eq!(mpt.get([1, 2, 3]), None);
}

#[test]
pub fn mpt_extension() {
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
    assert_eq!(
        mpt.hash_slow(),
        b256!("97aa4d930926792c6c5a716223c01dad6b64ce11ac261665d6f2fa031570ad26")
    );

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

#[test]
#[should_panic]
pub fn get_digest() {
    let mpt = MerkleTrie(Node::Digest(B256::ZERO));
    mpt.get([]);
}

#[test]
pub fn hash_sparse_mpt() {
    const NUM_LEAVES: usize = 1024;

    // populate leaves with hashed keys and RLP-encoded values
    let leaves: BTreeMap<_, _> = (0..NUM_LEAVES)
        .map(|i| {
            let key = U256::from(i);
            (
                Nibbles::unpack(keccak256(key.to_be_bytes::<32>())),
                alloy_rlp::encode(key),
            )
        })
        .collect();

    // generate proofs only for every second leaf
    let proof_keys = leaves.keys().step_by(2).cloned().collect();
    let mut hash_builder = HashBuilder::default().with_proof_retainer(proof_keys);
    for (key, value) in leaves {
        hash_builder.add_leaf(key, &value);
    }
    let root = hash_builder.root();
    let proofs = hash_builder.take_proofs();

    // reconstruct the trie from the RLP encoded proofs and verify the root hash
    let mpt = MerkleTrie::from_rlp_nodes(proofs.into_values())
        .expect("Failed to reconstruct Merkle Trie from proofs");
    assert_eq!(mpt.hash_slow(), root);
}

#[test]
pub fn parse_empty_proof() {
    let account_proof: Vec<Bytes> = Vec::new();

    let mpt = MerkleTrie::from_rlp_nodes(account_proof).unwrap();
    assert_eq!(mpt.hash_slow(), EMPTY_ROOT_HASH);
}
