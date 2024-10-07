use std::{collections::HashSet, ops::Range};

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

        let (added_nodes, removed_nodes) =
            difference(self.trie.to_rlp_nodes(), updated_trie.to_rlp_nodes());

        self.block_range = updated_range;
        self.trie = updated_trie;

        Ok(ChainUpdate {
            chain_info,
            added_nodes,
            removed_nodes,
        })
    }
}

fn difference(
    old: impl Iterator<Item = Bytes>,
    new: impl Iterator<Item = Bytes>,
) -> (Box<[Bytes]>, Box<[B256]>) {
    let old_set: HashSet<_> = old.collect();
    let new_set: HashSet<_> = new.collect();
    let added = new_set.difference(&old_set).cloned().collect();
    let removed = old_set.difference(&new_set).map(keccak256).collect();

    (added, removed)
}

pub struct ChainUpdate {
    pub chain_info: ChainInfo,
    pub added_nodes: Box<[Bytes]>,
    pub removed_nodes: Box<[B256]>,
}

#[derive(Error, Debug, PartialEq)]
pub enum ChainTrieError {
    #[error("Block already in range: {0}")]
    NewBlockInRange(u64),
}
