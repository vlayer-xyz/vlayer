use std::{
    collections::HashSet,
    fmt::{self},
    hash::Hash,
    path::Path,
};

use alloy_primitives::{keccak256, BlockNumber, ChainId, B256};
use alloy_rlp::{Decodable, RlpDecodable, RlpEncodable};
use bytes::Bytes;
use chain_common::ChainProofReceipt;
use chain_trie::UnverifiedChainTrie;
use derive_more::Debug;
use key_value::{Database, InMemoryDatabase, Mdbx, ReadTx, ReadWriteTx, WriteTx};
use proof_builder::{mpt_from_proofs, MerkleProofBuilder, ProofResult};

mod chain_trie;
mod db_node;
mod error;
mod proof_builder;
#[cfg(test)]
mod tests;

pub use chain_trie::ChainTrie;
use common::Hashable;
pub use db_node::DbNode;
pub use error::{ChainDbError, ChainDbResult};
pub use proof_builder::MerkleProof;
use u64_range::NonEmptyRange;

/// Merkle trie nodes table. Holds `node_hash -> rlp_node` mapping
const NODES: &str = "nodes";

/// Chains table. Holds `chain_id -> chain_info` mapping
const CHAINS: &str = "chains";

#[derive(Clone, PartialEq, Eq, RlpEncodable, RlpDecodable, Default, Debug)]
pub struct ChainInfo {
    first_block: BlockNumber,
    last_block: BlockNumber,
    root_hash: B256,
    #[debug("{zk_proof:#x}")]
    zk_proof: Bytes,
}

impl ChainInfo {
    pub const fn new(block_range: NonEmptyRange, root_hash: B256, zk_proof: Bytes) -> Self {
        Self {
            first_block: block_range.start(),
            last_block: block_range.end(),
            root_hash,
            zk_proof,
        }
    }

    pub fn into_parts(self) -> (NonEmptyRange, B256, Bytes) {
        (self.block_range(), self.root_hash, self.zk_proof)
    }

    pub fn block_range(&self) -> NonEmptyRange {
        // SAFETY: was created from `NonEmptyRange`
        NonEmptyRange::try_from_range(self.first_block..=self.last_block).unwrap()
    }
}

#[allow(dead_code)] // Used by Debug derive
fn slice_lower_hex<T: fmt::LowerHex>(slice: &[T]) -> impl fmt::LowerHex + '_ {
    struct SliceLowerHex<'a, T>(&'a [T]);

    impl<T: fmt::LowerHex> fmt::LowerHex for SliceLowerHex<'_, T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list()
                .entries(self.0.iter().map(|x| format!("{x:#x}")))
                .finish()
        }
    }

    SliceLowerHex(slice)
}

#[derive(Default, Clone, Eq, PartialEq, Debug)]
pub struct ChainUpdate {
    pub chain_info: ChainInfo,
    #[debug("{:#x}", slice_lower_hex(added_nodes))]
    pub added_nodes: Box<[Bytes]>,
    #[debug("{:#x}", slice_lower_hex(removed_nodes))]
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

    pub fn from_two_tries(
        range: NonEmptyRange,
        old: impl IntoIterator<Item = Bytes>,
        new: impl IntoIterator<Item = Bytes> + Hashable,
        receipt: &ChainProofReceipt,
    ) -> Result<Self, bincode::Error> {
        let chain_info = ChainInfo::new(range, new.hash_slow(), receipt.try_into()?);
        let (added_nodes, removed_nodes) = difference(old, new);
        Ok(Self::new(chain_info, added_nodes, removed_nodes))
    }
}

type DB = Box<dyn for<'a> Database<'a> + Send + Sync + 'static>;

pub enum Mode {
    ReadOnly,
    ReadWrite,
}

pub struct ChainDb {
    db: DB,
    mode: Mode,
}

impl ChainDb {
    #[must_use]
    pub fn in_memory() -> Self {
        let db = InMemoryDatabase::new();
        let mode = Mode::ReadWrite;
        Self::new(db, mode)
    }

    pub fn mdbx(path: impl AsRef<Path>, mode: Mode) -> ChainDbResult<Self> {
        let mut db = Mdbx::open(path)?;
        let mut tx = db.begin_rw()?;
        tx.create_table(NODES)?;
        tx.create_table(CHAINS)?;
        Box::new(tx).commit()?;
        Ok(Self::new(db, mode))
    }

    #[must_use]
    fn new(db: impl for<'a> Database<'a> + Send + Sync + 'static, mode: Mode) -> Self {
        Self {
            db: Box::new(db),
            mode,
        }
    }

    fn begin_ro(&self) -> ChainDbResult<ChainDbTx<dyn ReadTx + '_>> {
        let tx = self.db.begin_ro()?;
        Ok(ChainDbTx { tx })
    }

    fn begin_rw(&mut self) -> ChainDbResult<ChainDbTx<dyn ReadWriteTx + '_>> {
        match self.mode {
            Mode::ReadOnly => Err(ChainDbError::ReadOnly),
            Mode::ReadWrite => Ok(ChainDbTx {
                tx: self.db.begin_rw()?,
            }),
        }
    }

    pub fn get_chain_info(&self, chain_id: ChainId) -> ChainDbResult<Option<ChainInfo>> {
        self.begin_ro()?.get_chain_info(chain_id)
    }

    pub fn get_merkle_proof(&self, root_hash: B256, block_num: u64) -> ProofResult {
        self.begin_ro()?.get_merkle_proof(root_hash, block_num)
    }

    pub fn get_chain_proof(
        &self,
        chain_id: ChainId,
        block_numbers: impl IntoIterator<Item = BlockNumber>,
    ) -> ProofResult {
        self.begin_ro()?.get_chain_proof(chain_id, block_numbers)
    }

    // Does not verify ZK proof
    fn get_chain_trie_inner(
        &self,
        chain_id: ChainId,
    ) -> ChainDbResult<Option<UnverifiedChainTrie>> {
        let tx = self.begin_ro()?;
        let Some(chain_info) = self.get_chain_info(chain_id)? else {
            return Ok(None);
        };
        let (range, root_hash, zk_proof) = chain_info.into_parts();
        let chain_proof = (&zk_proof).try_into()?;

        let first_block_proof = tx.get_merkle_proof(root_hash, range.start())?;
        let last_block_proof = tx.get_merkle_proof(root_hash, range.end())?;
        let trie = mpt_from_proofs(first_block_proof, last_block_proof);

        Ok(Some(UnverifiedChainTrie::new(range, trie, chain_proof)))
    }

    pub fn get_chain_trie(&self, chain_id: ChainId) -> ChainDbResult<Option<ChainTrie>> {
        Ok(self
            .get_chain_trie_inner(chain_id)?
            .map(TryFrom::try_from) // Verifies ZK proof
            .transpose()?)
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
            tx.insert_node(&node)?;
        }

        Box::new(tx).commit()
    }
}

struct ChainDbTx<TX: ?Sized> {
    tx: Box<TX>,
}

impl<TX: ReadTx + ?Sized> ChainDbTx<TX> {
    pub fn get_chain_info(&self, chain_id: ChainId) -> ChainDbResult<Option<ChainInfo>> {
        let chain_id = chain_id.to_be_bytes();
        let chain_info = self
            .tx
            .get(CHAINS, &chain_id[..])?
            .map(|rlp| ChainInfo::decode(&mut &*rlp))
            .transpose()?;
        Ok(chain_info)
    }

    pub fn get_node(&self, node_hash: B256) -> ChainDbResult<DbNode> {
        let node_rlp = self
            .tx
            .get(NODES, &node_hash[..])?
            .ok_or(ChainDbError::NodeNotFound)?;
        let node = DbNode::decode(node_hash, node_rlp)?;
        Ok(node)
    }

    pub fn get_merkle_proof(&self, root_hash: B256, block_num: BlockNumber) -> ProofResult {
        MerkleProofBuilder::new(|node_hash| self.get_node(node_hash))
            .build_proof(root_hash, block_num)
    }

    pub fn get_chain_proof(
        &self,
        chain_id: ChainId,
        block_numbers: impl IntoIterator<Item = BlockNumber>,
    ) -> ProofResult {
        let chain_info = self
            .get_chain_info(chain_id)?
            .ok_or(ChainDbError::ChainNotFound(chain_id))?;
        let root_hash = chain_info.root_hash;
        let block_range = chain_info.block_range();

        let mut nodes = HashSet::new();
        for block_num in block_numbers {
            if !block_range.contains(block_num) {
                return Err(ChainDbError::BlockNumberOutsideRange {
                    block_num,
                    block_range,
                });
            }
            let merkle_proof = self.get_merkle_proof(root_hash, block_num)?;
            nodes.extend(merkle_proof.into_iter())
        }
        Ok(MerkleProof(nodes.into_iter().collect()))
    }
}

impl<TX: WriteTx + ?Sized> ChainDbTx<TX> {
    pub fn upsert_chain_info(
        &mut self,
        chain_id: ChainId,
        chain_info: &ChainInfo,
    ) -> ChainDbResult<()> {
        let chain_id = chain_id.to_be_bytes();
        let chain_info_rlp = alloy_rlp::encode(chain_info);
        self.tx.upsert(CHAINS, &chain_id[..], &chain_info_rlp[..])?;
        Ok(())
    }

    pub fn insert_node(&mut self, node_rlp: &Bytes) -> ChainDbResult<()> {
        let node_hash = keccak256(node_rlp);
        self.tx.insert(NODES, &node_hash[..], &node_rlp[..])?;
        Ok(())
    }

    pub fn delete_node(&mut self, node_hash: B256) -> ChainDbResult<()> {
        self.tx.delete(NODES, &node_hash[..])?;
        Ok(())
    }

    pub fn commit(self) -> ChainDbResult<()> {
        self.tx.commit()?;
        Ok(())
    }
}

pub fn difference<T>(
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
