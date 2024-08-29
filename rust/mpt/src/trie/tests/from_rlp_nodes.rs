use crate::{
    key_nibbles::KeyNibbles,
    node::{constructors::EMPTY_CHILDREN, Node},
    MerkleTrie,
};
use alloy_primitives::B256;

type EncodedNode = Vec<u8>;

fn rlp_encoded(root: &Node) -> Vec<EncodedNode> {
    // Encode the root node
    let mut out = vec![root.rlp_encoded()];
    // Encode the descendants of the root node
    match root {
        Node::Null | Node::Leaf(_, _) | Node::Digest(_) => {}
        Node::Extension(_, child) => out.extend(rlp_encoded(child)),
        Node::Branch(children, _) => {
            let non_empty_children = children.iter().flatten();
            let encoded_nodes = non_empty_children.flat_map(|c| rlp_encoded(c));
            out.extend(encoded_nodes);
        }
    }
    out
}

#[test]
fn null() {
    let mpt = MerkleTrie(Node::Null);
    let proof = rlp_encoded(&mpt.0);
    assert_eq!(mpt, MerkleTrie::from_rlp_nodes(proof).unwrap());
}

#[test]
fn digest() {
    let mpt = MerkleTrie(Node::Digest(B256::ZERO));
    let proof = rlp_encoded(&mpt.0);
    assert_eq!(mpt, MerkleTrie::from_rlp_nodes(proof).unwrap());
}

#[test]
fn leaf() {
    let key_nibbles: KeyNibbles = B256::ZERO.into();
    let mpt = MerkleTrie(Node::Leaf(key_nibbles, [0].into()));
    let proof = rlp_encoded(&mpt.0);
    assert_eq!(mpt, MerkleTrie::from_rlp_nodes(proof).unwrap());
}

#[test]
fn branch() {
    let mpt = MerkleTrie(Node::branch_with_value([42]));
    let proof = rlp_encoded(&mpt.0);

    let decoded_mpt = MerkleTrie::from_rlp_nodes(proof).unwrap();
    assert_eq!(mpt, decoded_mpt);
}
