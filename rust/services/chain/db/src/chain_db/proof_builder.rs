use std::ops::Deref;

use alloy_primitives::{BlockNumber, B256};
use alloy_rlp::Decodable;
use mpt::{Node, NodeRef};
use nybbles::Nibbles;

use super::{ChainDbError, ChainDbResult};

pub type ProofResult = ChainDbResult<Box<[(Option<B256>, Node)]>>;

pub struct MerkleProofBuilder<F: Fn(B256) -> ChainDbResult<Node>> {
    load_node: F,
    nodes: Vec<(Option<B256>, Node)>,
    nibbles: Nibbles,
}

impl<F: Fn(B256) -> ChainDbResult<Node>> MerkleProofBuilder<F> {
    pub fn new(load_node: F) -> Self {
        Self {
            load_node,
            nodes: Default::default(),
            nibbles: Default::default(),
        }
    }

    pub fn build_proof(mut self, root_hash: B256, block_num: BlockNumber) -> ProofResult {
        self.nibbles = Nibbles::unpack(alloy_rlp::encode(block_num));
        self.visit_node_hash(root_hash)
    }

    fn visit_node_hash(mut self, node_hash: B256) -> ProofResult {
        let node = (self.load_node)(node_hash)?;
        self.visit_node(Some(node_hash), node)
    }

    fn visit_node(mut self, node_hash: Option<B256>, node: Node) -> ProofResult {
        self.nodes.push((node_hash, node));
        match &self.nodes.last().expect("just pushed").1 {
            Node::Leaf(prefix, value) if *prefix == &*self.nibbles => self.finalize(),
            Node::Leaf(..) | Node::Null => Err(ChainDbError::BlockNotFound),
            Node::Extension(prefix, child) => {
                self.nibbles = strip_prefix(&self.nibbles, prefix.as_slice())
                    .ok_or(ChainDbError::BlockNotFound)?;
                let child_ref = NodeRef::from_node(child);
                self.visit_child_node(child_ref)
            }
            Node::Branch(children, _) => {
                let Some((idx, remaining)) = self.nibbles.split_first() else {
                    return self.finalize();
                };
                let idx = *idx as usize;
                self.nibbles = Nibbles::from_nibbles(remaining);
                let child = children[idx]
                    .as_deref()
                    .ok_or(ChainDbError::BlockNotFound)?;
                let child_ref = NodeRef::from_node(child);
                self.visit_child_node(child_ref)
            }
            Node::Digest(node_hash) => {
                let node_hash = *node_hash;
                self.visit_node_hash(node_hash)
            }
        }
    }

    fn visit_child_node(mut self, child_ref: NodeRef) -> ProofResult {
        match child_ref {
            NodeRef::Empty | NodeRef::Node(_) => Err(ChainDbError::InvalidNode),
            NodeRef::Digest(node_hash) => self.visit_node_hash(node_hash),
            NodeRef::InlineNode(node_rlp) => {
                let mut node_rlp: &[u8] = &node_rlp;
                let node = Node::decode(&mut node_rlp)?;
                self.visit_node(None, node)
            }
        }
    }

    fn finalize(self) -> ProofResult {
        Ok(self.nodes.into_boxed_slice())
    }
}

fn strip_prefix(nibbles: &Nibbles, prefix: impl AsRef<[u8]>) -> Option<Nibbles> {
    nibbles
        .as_slice()
        .strip_prefix(prefix.as_ref())
        .map(Nibbles::from_nibbles)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn nibbles_strip_prefix() -> Result<()> {
        let nibbles = Nibbles::new();
        assert_eq!(strip_prefix(&nibbles, &*Nibbles::unpack([0])), None);

        let nibbles = Nibbles::unpack([0, 1]);
        assert_eq!(strip_prefix(&nibbles, []).unwrap(), nibbles);
        assert_eq!(strip_prefix(&nibbles, &*Nibbles::unpack([0])).unwrap(), Nibbles::unpack([1]));

        Ok(())
    }
}
