use super::ParseNodeError;
use crate::node::Node;
use alloy_primitives::{keccak256, B256};
use rlp as legacy_rlp;
use std::collections::HashMap;

/// Returns the decoded node and its RLP hash.
pub(crate) fn parse_node(rlp: impl AsRef<[u8]>) -> Result<(Option<B256>, Node), ParseNodeError> {
    let rlp = rlp.as_ref();
    let node = legacy_rlp::decode(rlp)?;
    // the hash is only needed for RLP length >= 32
    Ok(((rlp.len() >= 32).then(|| keccak256(rlp)), node))
}

pub(crate) fn resolve_trie(root: Node, nodes_by_hash: &HashMap<B256, Node>) -> Node {
    match root {
        Node::Null | Node::Leaf(..) => root,
        Node::Extension(prefix, child) => {
            Node::Extension(prefix, Box::new(resolve_trie(*child, nodes_by_hash)))
        }
        Node::Branch(mut children) => {
            // iterate over the children in place, resolving each child node recursively.
            for child in children.iter_mut() {
                if let Some(node) = child.take() {
                    *child = Some(Box::new(resolve_trie(*node, nodes_by_hash)));
                }
            }
            Node::Branch(children)
        }
        Node::Digest(digest) => match nodes_by_hash.get(&digest) {
            Some(node) => resolve_trie(node.clone(), nodes_by_hash),
            None => root,
        },
    }
}

#[cfg(test)]
mod parse_node {
    use alloy_primitives::{b256, B256};
    use nybbles::Nibbles;

    use super::parse_node;
    use crate::node::Node;

    #[test]
    fn inline() -> anyhow::Result<()> {
        let node = Node::Null;
        let (hash, node) = parse_node(&node.rlp_encoded())?;
        assert_eq!(hash, None);
        assert_eq!(node, Node::Null);
        Ok(())
    }

    #[test]
    fn non_inline() -> anyhow::Result<()> {
        let nibbles = Nibbles::unpack(B256::ZERO);
        let node = Node::Leaf(nibbles.clone(), vec![0].into());
        let (hash, node) = parse_node(&node.rlp_encoded())?;
        assert_eq!(
            hash,
            Some(b256!(
                "ebcd1aff3f48f44a89c8bceb54a7e73c44edda96852b9debc4447b5ac9be19a6"
            ))
        );
        assert_eq!(node, Node::Leaf(nibbles, vec![0].into()));
        Ok(())
    }
}
