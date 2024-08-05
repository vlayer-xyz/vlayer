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

#[cfg(test)]
mod resolve_trie {
    use std::array::from_fn;

    use alloy_primitives::keccak256;
    use alloy_trie::HashMap;
    use nybbles::Nibbles;

    use crate::node::Node;

    use super::resolve_trie;

    #[test]
    fn null() {
        let root = Node::Null;
        let nodes_by_hash = HashMap::new();
        let resolved_node = resolve_trie(root, &nodes_by_hash);
        assert_eq!(resolved_node, Node::Null);
    }

    #[test]
    fn leaf() {
        let nibbles = Nibbles::from_nibbles([0]);
        let root = Node::Leaf(nibbles.clone(), vec![0].into());
        let nodes_by_hash = HashMap::new();
        let resolved_node = resolve_trie(root, &nodes_by_hash);
        assert_eq!(resolved_node, Node::Leaf(nibbles, vec![0].into()));
    }

    #[test]
    fn digest() {
        let null_node = Node::Null;
        let digest = keccak256(null_node.rlp_encoded());
        let node = Node::Digest(digest);
        let nodes_by_hash = HashMap::new();
        let resolved_node = resolve_trie(node.clone(), &nodes_by_hash);
        assert_eq!(resolved_node, node);
    }

    #[test]
    fn digest_resolved() {
        let null_node = Node::Null;
        let digest = keccak256(null_node.rlp_encoded());
        let node = Node::Digest(digest);
        let nodes_by_hash = HashMap::from_iter([(digest, null_node.clone())]);
        let resolved_node = resolve_trie(node, &nodes_by_hash);
        assert_eq!(resolved_node, Node::Null);
    }

    #[test]
    fn extension() {
        let leaf_nibbles = Nibbles::from_nibbles([1]);
        let extension_nibbles = Nibbles::from_nibbles([0]);
        let leaf = Node::Leaf(leaf_nibbles.clone(), vec![0].into());
        let digest = keccak256(leaf.rlp_encoded());
        let extension = Node::Extension(extension_nibbles.clone(), Box::new(Node::Digest(digest)));
        let nodes_by_hash = HashMap::from([(digest, leaf.clone())]);
        let resolved_node = resolve_trie(extension, &nodes_by_hash);
        assert_eq!(
            resolved_node,
            Node::Extension(extension_nibbles, Box::new(leaf))
        );
    }

    #[test]
    fn branch() {
        let leaf_nibbles = Nibbles::from_nibbles([1]);
        let leaf = Node::Leaf(leaf_nibbles.clone(), vec![0].into());
        let digest = keccak256(leaf.rlp_encoded());
        let child = None;
        let mut children: [_; 16] = from_fn(|_| child.clone());
        children[0] = Some(Box::new(Node::Digest(digest)));
        let branch = Node::Branch(children);
        let nodes_by_hash = HashMap::from([(digest, leaf.clone())]);
        let resolved_node = resolve_trie(branch, &nodes_by_hash);
        let Node::Branch(children) = resolved_node else {
            panic!("expected branch, got {:?}", resolved_node);
        };
        assert_eq!(children[0], Some(Box::new(leaf)));
    }
}
