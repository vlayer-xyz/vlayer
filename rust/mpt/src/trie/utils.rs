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
        Node::Branch(mut children, value) => {
            // iterate over the children in place, resolving each child node recursively.
            for child in &mut children {
                if let Some(node) = child.take() {
                    *child = Some(Box::new(resolve_trie(*node, nodes_by_hash)));
                }
            }
            Node::Branch(children, value)
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

    use super::parse_node;
    use crate::{key_nibbles::KeyNibbles, node::Node};

    #[test]
    fn inline() -> anyhow::Result<()> {
        let node = Node::Null;
        let (hash, parsed_node) = parse_node(node.rlp_encoded())?;
        assert_eq!(hash, None);
        assert_eq!(parsed_node, node);
        Ok(())
    }

    #[test]
    fn non_inline() -> anyhow::Result<()> {
        let nibbles = KeyNibbles::unpack(B256::ZERO);
        let node = Node::Leaf(nibbles, [0].into());
        let (hash, parsed_node) = parse_node(node.rlp_encoded())?;
        assert_eq!(
            hash,
            Some(b256!("ebcd1aff3f48f44a89c8bceb54a7e73c44edda96852b9debc4447b5ac9be19a6"))
        );
        assert_eq!(parsed_node, node);
        Ok(())
    }

    #[test]
    fn branch_value_is_rlp_encoded() -> anyhow::Result<()> {
        let node = Node::branch_with_value([0]);
        let (hash, parsed_node) = parse_node(node.rlp_encoded())?;
        assert_eq!(hash, None);
        assert_eq!(parsed_node, node);
        Ok(())
    }
}

#[cfg(test)]
mod resolve_trie {
    use alloy_primitives::keccak256;
    use alloy_trie::HashMap;

    use crate::{key_nibbles::KeyNibbles, node::Node};

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
        let key_nibbles: KeyNibbles = [0].into();
        let root = Node::Leaf(key_nibbles.clone(), [0].into());
        let nodes_by_hash = HashMap::new();
        let resolved_node = resolve_trie(root, &nodes_by_hash);
        assert_eq!(resolved_node, Node::Leaf(key_nibbles, [0].into()));
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
        let nodes_by_hash = HashMap::from_iter([(digest, null_node)]);
        let resolved_node = resolve_trie(node, &nodes_by_hash);
        assert_eq!(resolved_node, Node::Null);
    }

    #[test]
    fn extension() {
        let extension_nibbles: KeyNibbles = [0].into();
        let leaf = Node::Leaf([1].into(), [0].into());
        let digest = keccak256(leaf.rlp_encoded());
        let extension = Node::Extension(extension_nibbles.clone(), Box::new(Node::Digest(digest)));
        let nodes_by_hash = HashMap::from([(digest, leaf.clone())]);
        let resolved_node = resolve_trie(extension, &nodes_by_hash);
        assert_eq!(resolved_node, Node::Extension(extension_nibbles, Box::new(leaf)));
    }

    #[test]
    fn branch() {
        let leaf = Node::leaf([1], [0]);
        let digest = keccak256(leaf.rlp_encoded());
        let branch = Node::branch_with_child(0, Node::Digest(digest));
        let nodes_by_hash = HashMap::from([(digest, leaf.clone())]);
        let resolved_node = resolve_trie(branch, &nodes_by_hash);
        let Node::Branch(children, None) = resolved_node else {
            panic!("expected branch, got {:?}", resolved_node);
        };

        assert_eq!(children[0], Some(Box::new(leaf)));
    }

    #[test]
    fn branch_with_value() {
        let branch = Node::branch_with_value([42]);
        let nodes_by_hash = HashMap::new();
        let resolved_node = resolve_trie(branch, &nodes_by_hash);
        let Node::Branch(_, Some(value)) = resolved_node else {
            panic!("expected branch with value, got {:?}", resolved_node);
        };

        assert_eq!(value, [42u8].into());
    }
}
