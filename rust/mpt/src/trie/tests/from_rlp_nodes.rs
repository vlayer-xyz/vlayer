use crate::{node::Node, MerkleTrie};
use alloy_primitives::B256;
use nybbles::Nibbles;

type EncodedNode = Vec<u8>;

fn rlp_encoded(root: &Node) -> Vec<EncodedNode> {
    // Encode the root node
    let mut out = vec![root.rlp_encoded()];
    // Encode the descendants of the root node
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

#[test]
fn branch() {
    let children: [Option<Box<Node>>; 16] = Default::default();
    let value = Some(vec![42u8].into());
    let mpt = MerkleTrie(Node::Branch(children, value.clone()));
    let proof = rlp_encoded(&mpt.0);

    // Decode the proof back to a MerkleTrie and check for equality
    let decoded_mpt = MerkleTrie::from_rlp_nodes(proof).unwrap();
    assert_eq!(mpt, decoded_mpt);
}
