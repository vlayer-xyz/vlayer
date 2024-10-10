use std::{
    collections::HashSet,
    hash::Hash,
    ops::{Deref, DerefMut, Range, RangeInclusive},
};

use alloy_primitives::{keccak256, ChainId, B256};
use alloy_rlp::{Bytes as RlpBytes, BytesMut, Decodable, Encodable, RlpDecodable, RlpEncodable};
use block_trie::BlockTrie;
use bytes::Bytes;
use mpt::{KeyNibbles, MerkleTrie, Node, NodeRef, EMPTY_ROOT_HASH};
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

#[derive(Debug, Clone, PartialEq, Eq, RlpEncodable, RlpDecodable, Default)]
pub struct ChainInfo {
    pub first_block: u64,
    pub last_block: u64,
    pub root_hash: B256,
    pub zk_proof: RlpBytes,
}

impl ChainInfo {
    pub fn new(
        block_range: RangeInclusive<u64>,
        root_hash: B256,
        zk_proof: impl Into<Bytes>,
    ) -> Self {
        Self {
            first_block: *block_range.start(),
            last_block: *block_range.end(),
            root_hash,
            zk_proof: zk_proof.into(),
        }
    }

    pub fn block_range(&self) -> RangeInclusive<u64> {
        self.first_block..=self.last_block
    }
}

#[derive(Debug, Default, Clone)]
pub struct ChainUpdate {
    pub chain_info: ChainInfo,
    pub added_nodes: Box<[Bytes]>,
    pub removed_nodes: Box<[Bytes]>,
}

impl ChainUpdate {
    pub fn new(
        chain_info: ChainInfo,
        added_nodes: impl IntoIterator<Item = Bytes>,
        removed_nodes: impl IntoIterator<Item = Bytes>,
    ) -> Self {
        Self {
            chain_info,
            added_nodes: added_nodes.into_iter().collect(),
            removed_nodes: removed_nodes.into_iter().collect(),
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
        self.begin_ro()?.get_merkle_proof(root_hash, block_num)
    }

    pub fn get_chain_trie(&self, chain_id: ChainId) -> ChainDbResult<Option<ChainTrie>> {
        let tx = self.begin_ro()?;
        let Some(chain_info) = self.get_chain_info(chain_id)? else {
            return Ok(None);
        };
        let ChainInfo {
            root_hash,
            first_block,
            last_block,
            ..
        } = chain_info;

        let first_block_proof = tx.get_merkle_proof(root_hash, first_block)?;
        let last_block_proof = tx.get_merkle_proof(root_hash, last_block)?;
        let trie: MerkleTrie = first_block_proof
            .into_vec()
            .into_iter()
            .chain(last_block_proof)
            .collect();

        Ok(Some(ChainTrie::new(chain_info.block_range(), trie)))
    }

    pub fn update_chain(
        &mut self,
        chain_id: ChainId,
        ChainUpdate {
            chain_info,
            added_nodes,
            removed_nodes,
        }: ChainUpdate,
    ) -> ChainDbResult<()> {
        let mut tx = self.begin_rw()?;

        tx.upsert_chain_info(chain_id, &chain_info)?;

        for node in removed_nodes {
            tx.delete_node(keccak256(node))?;
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

fn difference<T>(
    old: impl IntoIterator<Item = T>,
    new: impl IntoIterator<Item = T>,
) -> (Box<[T]>, Box<[T]>)
where
    T: Eq + Clone + Hash,
{
    let old_set: HashSet<_> = old.into_iter().collect();
    let new_set: HashSet<_> = new.into_iter().collect();
    let added = new_set.difference(&old_set).cloned().collect();
    let removed = old_set.difference(&new_set).cloned().collect();

    (added, removed)
}

pub struct ChainTrie {
    block_range: RangeInclusive<u64>,
    trie: BlockTrie,
}

impl ChainTrie {
    pub fn new(block_range: RangeInclusive<u64>, trie: impl Into<BlockTrie>) -> Self {
        Self {
            block_range,
            trie: trie.into(),
        }
    }
}
