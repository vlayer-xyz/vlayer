use std::ops::{Deref, DerefMut, Range};

use alloy_primitives::{keccak256, ChainId, B256};
use alloy_rlp::{Bytes as RlpBytes, BytesMut, Decodable, Encodable, RlpDecodable, RlpEncodable};
use bytes::Bytes;
use mpt::{KeyNibbles, Node, NodeRef, EMPTY_ROOT_HASH};
use nybbles::Nibbles;
use proof_builder::{MerkleProofBuilder, ProofResult};
use thiserror::Error;

use crate::{Database, DbError, DbResult, ReadTx, WriteTx};

mod proof_builder;
#[cfg(test)]
mod tests;

/// Merkle trie nodes table. Holds `node_hash -> rlp_node` mapping
const NODES: &str = "nodes";

/// Chains table. Holds `chain_id -> chain_info` mapping
const CHAINS: &str = "chains";

#[derive(Debug, Clone, PartialEq, Eq, RlpEncodable, RlpDecodable)]
pub struct ChainInfo {
    pub first_block: u64,
    pub last_block: u64,
    pub merkle_root: B256,
    pub zk_proof: RlpBytes,
}

impl ChainInfo {
    pub fn new(range: Range<u64>, merkle_root: B256, zk_proof: impl Into<Bytes>) -> Self {
        Self {
            first_block: range.start,
            last_block: range.end - 1,
            merkle_root,
            zk_proof: zk_proof.into(),
        }
    }
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

    pub fn get_merkle_proof(&self, root_hash: B256, block_num: u64) -> ProofResult {
        let tx = self.begin_ro()?;
        let proof_builder = MerkleProofBuilder::new(|node_hash| tx.get_node(node_hash));
        proof_builder.build_proof(root_hash, block_num)
    }

    pub fn update_chain(
        &mut self,
        chain_id: ChainId,
        chain_info: &ChainInfo,
        removed_nodes: impl IntoIterator<Item = B256>,
        added_nodes_rlp: impl IntoIterator<Item = Bytes>,
    ) -> ChainDbResult<()> {
        let mut tx = self.begin_rw()?;

        tx.upsert_chain_info(chain_id, chain_info)?;

        for node_hash in removed_nodes {
            tx.delete_node(node_hash)?;
        }

        for node in added_nodes_rlp {
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

    pub fn get_merkle_proof(&self, root_hash: B256, block_num: u64) -> ProofResult {
        MerkleProofBuilder::new(|node_hash| self.get_node(node_hash))
            .build_proof(root_hash, block_num)
    }
}

impl<TX: WriteTx> ChainDbTx<TX> {
    pub fn upsert_chain_info(
        &mut self,
        chain_id: ChainId,
        chain_info: &ChainInfo,
    ) -> ChainDbResult<()> {
        let chain_id = chain_id.to_be_bytes();
        let chain_info_rlp = alloy_rlp::encode(chain_info);
        self.tx.upsert(CHAINS, chain_id, chain_info_rlp)?;
        Ok(())
    }

    pub fn insert_node(&mut self, node_rlp: Bytes) -> ChainDbResult<()> {
        let node_hash = keccak256(&node_rlp);
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
