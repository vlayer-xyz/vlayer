use std::ops::{Deref, DerefMut};

use alloy_primitives::{keccak256, B256};
use alloy_rlp::{BytesMut, Decodable, Encodable, RlpDecodable, RlpEncodable};
use mpt::{KeyNibbles, Node, NodeRef, EMPTY_ROOT_HASH};
use thiserror::Error;

use crate::{Database, DbError, DbResult, ReadTx, WriteTx};

#[cfg(test)]
mod tests;

/// Merkle trie nodes table. Holds `node_hash -> rlp_node` mapping
const NODES: &str = "nodes";

/// Chains table. Holds `chain_id -> chain_info` mapping
const CHAINS: &str = "chains";

type ChainId = u64;

#[derive(Debug, Clone, PartialEq, Eq, RlpEncodable, RlpDecodable)]
pub struct ChainInfo {
    pub first_block: u64,
    pub last_block: u64,
    pub merkle_root: B256,
    pub zk_proof: Vec<u8>,
}

pub struct ChainDb<DB: for<'a> Database<'a>> {
    db: DB,
}

#[derive(Error, Debug, PartialEq)]
pub enum ChainDbError {
    #[error("Database error: {0}")]
    Db(#[from] DbError),
    #[error("RLP error: {0}")]
    Node(#[from] alloy_rlp::Error),
    #[error("Node not found")]
    NodeNotFound,
    #[error("Invalid node")]
    InvalidNode,
    #[error("Block not found")]
    BlockNotFound,
}

pub type ChainDbResult<T> = Result<T, ChainDbError>;

impl<DB: for<'a> Database<'a>> ChainDb<DB> {
    pub fn new(db: DB) -> Self {
        Self { db }
    }

    fn begin_ro(&self) -> ChainDbResult<ChainDbTx<<DB as Database<'_>>::ReadTx>> {
        let tx = self.db.begin_ro()?;
        Ok(ChainDbTx { tx })
    }

    fn begin_rw(&mut self) -> ChainDbResult<ChainDbTx<<DB as Database<'_>>::ReadWriteTx>> {
        let tx = self.db.begin_rw()?;
        Ok(ChainDbTx { tx })
    }

    pub fn get_chain_info(&self, chain_id: ChainId) -> ChainDbResult<Option<ChainInfo>> {
        self.begin_ro()?.get_chain_info(chain_id)
    }

    pub fn get_merkle_proof(&self, root_hash: B256, block_num: u64) -> ChainDbResult<Box<[Node]>> {
        let tx = self.begin_ro()?;
        let mut node_hash = root_hash;
        let mut nodes = vec![];
        let mut key_nibbles = KeyNibbles::unpack(alloy_rlp::encode(block_num));
        let mut nibbles: &[u8] = key_nibbles.as_ref();

        loop {
            let node = tx.get_node(node_hash)?;

            // TODO: Traverse MPT

            nodes.push(node);
        }

        Ok(nodes.into_boxed_slice())
    }

    pub fn update_chain<'a>(
        &mut self,
        chain_id: ChainId,
        chain_info: &ChainInfo,
        removed_nodes: impl IntoIterator<Item = B256>,
        added_nodes: impl IntoIterator<Item = &'a Node>,
    ) -> ChainDbResult<()> {
        let mut tx = self.begin_rw()?;

        tx.insert_chain_info(chain_id, chain_info)?;

        for node_hash in removed_nodes {
            tx.delete_node(node_hash)?;
        }

        for node in added_nodes {
            tx.insert_node(node)?;
        }

        tx.commit()
    }
}

struct ChainDbTx<TX> {
    tx: TX,
}

impl<TX: ReadTx> ChainDbTx<TX> {
    pub fn get_chain_info(&self, chain_id: ChainId) -> ChainDbResult<Option<ChainInfo>> {
        let chain_id = chain_id.to_be_bytes();
        let chain_info = self
            .tx
            .get(CHAINS, chain_id)?
            .map(|rlp| ChainInfo::decode(&mut rlp.deref()))
            .transpose()?;
        Ok(chain_info)
    }

    pub fn get_node(&self, node_hash: B256) -> ChainDbResult<Node> {
        let node_rlp = self
            .tx
            .get(NODES, node_hash)?
            .ok_or(ChainDbError::NodeNotFound)?;
        let node = Node::decode(&mut node_rlp.as_ref())?;
        Ok(node)
    }
}

impl<TX: WriteTx> ChainDbTx<TX> {
    pub fn insert_chain_info(
        &mut self,
        chain_id: ChainId,
        chain_info: &ChainInfo,
    ) -> ChainDbResult<()> {
        let chain_id = chain_id.to_be_bytes();
        let chain_info_rlp = alloy_rlp::encode(chain_info);
        self.tx.insert(CHAINS, chain_id, chain_info_rlp)?;
        Ok(())
    }

    pub fn insert_node(&mut self, node: &Node) -> ChainDbResult<()> {
        let node_rlp = node.rlp_encoded();
        let node_hash = node_hash(node, &node_rlp);
        self.tx.insert(NODES, node_hash, node_rlp)?;
        Ok(())
    }

    pub fn delete_node(&mut self, node_hash: B256) -> ChainDbResult<()> {
        self.tx.delete(NODES, node_hash)?;
        Ok(())
    }

    pub fn commit(self) -> ChainDbResult<()> {
        self.tx.commit()?;
        Ok(())
    }
}

#[inline]
fn node_hash(node: &Node, node_rlp: impl AsRef<[u8]>) -> B256 {
    match node {
        Node::Null => EMPTY_ROOT_HASH,
        Node::Digest(digest) => *digest,
        _ => keccak256(node_rlp.as_ref()),
    }
}
