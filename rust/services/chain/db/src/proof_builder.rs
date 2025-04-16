use alloy_primitives::{B256, BlockNumber};
use mpt::{Sha2Node as Node, Sha2NodeRef as NodeRef, Sha2Trie as MerkleTrie};
use nybbles::Nibbles;

use crate::{ChainDbError, ChainDbResult, DbNode};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MerkleProof(pub Box<[DbNode]>);

impl IntoIterator for MerkleProof {
    type IntoIter = <Vec<DbNode> as IntoIterator>::IntoIter;
    type Item = DbNode;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_vec().into_iter()
    }
}

pub fn mpt_from_proofs(lhs: MerkleProof, rhs: MerkleProof) -> MerkleTrie {
    // Cannot be done by From<(MerkleProof, MerkleProof)> because of orphan rule
    lhs.into_iter()
        .chain(rhs)
        .map(|db_node| (db_node.hash, db_node.node))
        .collect()
}

impl From<MerkleProof> for MerkleTrie {
    fn from(proof: MerkleProof) -> Self {
        proof
            .into_iter()
            .map(|db_node| (db_node.hash, db_node.node))
            .collect()
    }
}

pub type ProofResult = ChainDbResult<MerkleProof>;

pub struct MerkleProofBuilder<F: Fn(B256) -> ChainDbResult<DbNode>> {
    load_node: F,
    nodes: Vec<DbNode>,
    nibbles: Nibbles,
}

impl<F: Fn(B256) -> ChainDbResult<DbNode>> MerkleProofBuilder<F> {
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

    fn visit_node_hash(self, node_hash: B256) -> ProofResult {
        let node = (self.load_node)(node_hash)?;
        self.visit_node(node)
    }

    #[allow(clippy::expect_used)]
    fn visit_node(mut self, node: DbNode) -> ProofResult {
        self.nodes.push(node);
        match &self.nodes.last().expect("just pushed").node {
            Node::Leaf(prefix, _) if prefix == &*self.nibbles => self.finalize(),
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
            Node::_Phantom(_) => unreachable!(),
        }
    }

    fn visit_child_node(self, child_ref: NodeRef) -> ProofResult {
        match child_ref {
            NodeRef::Empty | NodeRef::Node(_) => Err(ChainDbError::InvalidNode),
            NodeRef::Digest(node_hash) => self.visit_node_hash(node_hash),
            NodeRef::InlineNode(node_rlp) => {
                let node = DbNode::decode(None, node_rlp)?;
                self.visit_node(node)
            }
            NodeRef::_Phantom(_) => unreachable!(),
        }
    }

    fn finalize(self) -> ProofResult {
        Ok(MerkleProof(self.nodes.into_boxed_slice()))
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
    use anyhow::Result;

    use super::*;

    #[test]
    fn nibbles_strip_prefix() -> Result<()> {
        let nibbles = Nibbles::new();
        assert_eq!(strip_prefix(&nibbles, &*Nibbles::from_nibbles([0])), None);

        let nibbles = Nibbles::from_nibbles([0, 1]);
        assert_eq!(strip_prefix(&nibbles, []).unwrap(), nibbles);
        assert_eq!(
            strip_prefix(&nibbles, &*Nibbles::from_nibbles([0])).unwrap(),
            Nibbles::from_nibbles([1])
        );

        Ok(())
    }
}
