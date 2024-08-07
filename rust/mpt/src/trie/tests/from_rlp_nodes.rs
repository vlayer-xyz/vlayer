use crate::{node::Node, MerkleTrie};
use alloy_primitives::B256;
use nybbles::Nibbles;

fn rlp_encoded(root: &Node) -> Vec<Vec<u8>> {
    let mut out = vec![root.rlp_encoded()];
    match root {
        Node::Null | Node::Leaf(_, _) | Node::Digest(_) => {}
        Node::Extension(_, child) => out.extend(rlp_encoded(child)),
        Node::Branch(children, _) => {
            out.extend(children.iter().flatten().flat_map(|c| rlp_encoded(c)));
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
    let mpt = MerkleTrie(Node::Leaf(Nibbles::unpack(B256::ZERO), vec![0].into()));
    let proof = rlp_encoded(&mpt.0);
    assert_eq!(mpt, MerkleTrie::from_rlp_nodes(proof).unwrap());
}
