use std::{collections::HashSet, hash::Hash, ops::Range};

use alloy_primitives::{keccak256, B256};
use bytes::Bytes;
use chain_engine::BlockTrie;
use thiserror::Error;

use super::ChainInfo;

pub struct ChainTrie {
    block_range: Range<u64>,
    trie: BlockTrie,
}

impl ChainTrie {
    pub fn new(block_range: Range<u64>, trie: impl Into<BlockTrie>) -> Self {
        Self {
            block_range,
            trie: trie.into(),
        }
    }

    pub fn update(
        &mut self,
        new_blocks: impl IntoIterator<Item = (u64, B256)>,
        zk_proof: impl Into<Bytes>,
    ) -> Result<ChainUpdate, ChainTrieError> {
        let mut updated_trie = self.trie.clone();
        let mut updated_range = self.block_range.clone();

        // TODO: Enforce that block range is contiguous
        for (block_num, block_hash) in new_blocks {
            if self.block_range.contains(&block_num) {
                return Err(ChainTrieError::NewBlockInRange(block_num));
            }
            updated_range.start = std::cmp::min(updated_range.start, block_num);
            updated_range.end = std::cmp::max(updated_range.end, block_num);

            updated_trie.insert(block_num, &block_hash);
        }

        let root_hash = updated_trie.hash_slow();
        let chain_info = ChainInfo::new(updated_range.clone(), root_hash, zk_proof);

        let (added_nodes, removed_nodes) = difference(&self.trie, &updated_trie);

        self.block_range = updated_range;
        self.trie = updated_trie;

        Ok(ChainUpdate::new(chain_info, added_nodes, removed_nodes))
    }
}

fn difference<T: Eq + Clone + Hash>(
    old: impl IntoIterator<Item = T>,
    new: impl IntoIterator<Item = T>,
) -> (Box<[T]>, Box<[T]>) {
    let old_set: HashSet<_> = old.into_iter().collect();
    let new_set: HashSet<_> = new.into_iter().collect();
    let added = new_set.difference(&old_set).cloned().collect();
    let removed = old_set.difference(&new_set).cloned().collect();

    (added, removed)
}

#[derive(Debug, Default)]
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

#[derive(Error, Debug, PartialEq)]
pub enum ChainTrieError {
    #[error("Block already in range: {0}")]
    NewBlockInRange(u64),
}
