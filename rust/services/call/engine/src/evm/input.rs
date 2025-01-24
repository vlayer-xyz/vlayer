use std::{collections::HashMap, iter::once};

use alloy_primitives::{BlockHash, BlockNumber, Bytes, ChainId, B256};
use block_header::{EthBlockHeader, EvmBlockHeader, Hashable};
use derivative::Derivative;
use derive_more::{Deref, DerefMut, From, Into, IntoIterator};
use derive_new::new;
use itertools::Itertools;
use mpt::KeccakMerkleTrie as MerkleTrie;
use serde::{Deserialize, Serialize};
use tracing::debug;

use super::env::location::ExecutionLocation;

/// The serializable input to derive and validate a [EvmEnv].
#[derive(Debug, Serialize, Derivative, Deserialize, Clone)]
#[derivative(Default)]
pub struct EvmInput {
    #[derivative(Default(value = "Box::new(EthBlockHeader::default())"))]
    pub header: Box<dyn EvmBlockHeader>,
    pub state_trie: MerkleTrie,
    pub storage_tries: Vec<MerkleTrie>,
    pub contracts: Vec<Bytes>,
    pub ancestors: Vec<Box<dyn EvmBlockHeader>>,
}

impl EvmInput {
    pub fn print_sizes(&self) {
        let total_storage_size: usize = self.storage_tries.iter().map(MerkleTrie::size).sum();

        debug!("state size: {}", self.state_trie.size());
        debug!("storage tries: {}", self.storage_tries.len());
        debug!("total storage size: {}", total_storage_size);
        debug!("contracts: {}", self.contracts.len());
        debug!("blocks: {}", self.ancestors.len());
    }
}

impl EvmInput {
    pub fn block_hashes(&self) -> HashMap<u64, B256> {
        self.ancestors
            .iter()
            .map(|ancestor| (ancestor.number(), ancestor.hash_slow()))
            .chain(once((self.header.number(), self.header.hash_slow())))
            .collect()
    }

    pub fn assert_coherency(&self) {
        self.assert_state_root_coherency();
        self.assert_ancestors_coherency();
    }

    fn assert_state_root_coherency(&self) {
        let state_root = self.state_trie.hash_slow();
        assert_eq!(self.header.state_root(), &state_root, "State root mismatch");
    }

    fn assert_ancestors_coherency(&self) {
        let mut previous_header = &self.header;
        for ancestor in &self.ancestors {
            let ancestor_hash = ancestor.hash_slow();
            assert_eq!(
                previous_header.parent_hash(),
                &ancestor_hash,
                "Invalid chain: block {} is not the parent of block {}",
                ancestor.number(),
                previous_header.number()
            );
            previous_header = ancestor;
        }
    }
}

#[derive(Debug, Clone, From, Deref, DerefMut, IntoIterator)]
pub struct BlocksByChain(HashMap<ChainId, Vec<(BlockNumber, BlockHash)>>);

impl BlocksByChain {
    pub fn chain_ids(&self) -> Box<[ChainId]> {
        self.0.keys().cloned().collect()
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, From, Into, IntoIterator, new)]
pub struct MultiEvmInput {
    pub inputs: HashMap<ExecutionLocation, EvmInput>,
}

impl MultiEvmInput {
    pub fn from_entries<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (ExecutionLocation, EvmInput)>,
    {
        let inputs = iter.into_iter().collect();
        Self { inputs }
    }

    pub fn contains_time_travel(&self) -> bool {
        self.block_nums_by_chain()
            .values()
            .any(|block_nums| block_nums.len() > 1)
    }

    pub fn contains_teleport(&self) -> bool {
        self.block_nums_by_chain().len() > 1
    }

    pub fn assert_coherency(&self) {
        self.inputs.values().for_each(EvmInput::assert_coherency);
    }

    fn group_blocks<F, T>(&self, f: F) -> HashMap<ChainId, Vec<T>>
    where
        F: Fn(&ExecutionLocation, &EvmInput) -> T,
    {
        self.inputs
            .iter()
            .map(|(loc, evm_input)| (loc.chain_id, f(loc, evm_input)))
            .into_group_map()
    }

    pub fn blocks_by_chain(&self) -> BlocksByChain {
        self.group_blocks(|loc, evm_input| (loc.block_number, evm_input.header.hash_slow()))
            .into()
    }

    pub fn block_nums_by_chain(&self) -> HashMap<ChainId, Vec<BlockNumber>> {
        self.group_blocks(|loc, _| loc.block_number)
    }
}

impl FromIterator<(ExecutionLocation, EvmInput)> for MultiEvmInput {
    fn from_iter<I: IntoIterator<Item = (ExecutionLocation, EvmInput)>>(iter: I) -> Self {
        let inputs = iter.into_iter().collect();
        Self { inputs }
    }
}

#[cfg(test)]
mod test {
    use block_header::{EthBlockHeader, Hashable};
    use mpt::EMPTY_ROOT_HASH;

    use super::EvmInput;

    mod block_hashes {
        use super::*;

        #[test]
        fn success() {
            let ancestor = EthBlockHeader::default();
            let input = EvmInput {
                ancestors: vec![Default::default()],
                header: Box::new(EthBlockHeader {
                    number: 1,
                    parent_hash: ancestor.hash_slow(),
                    ..Default::default()
                }),
                ..Default::default()
            };
            let block_hashes = input.block_hashes();

            assert_eq!(block_hashes.len(), 2);
            assert_eq!(block_hashes.get(&0).unwrap(), &ancestor.hash_slow());
            assert_eq!(block_hashes.get(&1).unwrap(), &input.header.hash_slow());
        }
    }

    mod assert_state_root_coherency {
        use super::*;

        #[test]
        fn success() {
            let input = EvmInput {
                ancestors: vec![Default::default()],
                header: Box::new(EthBlockHeader {
                    state_root: EMPTY_ROOT_HASH,
                    ..Default::default()
                }),
                ..Default::default()
            };
            input.assert_state_root_coherency();
        }

        #[test]
        #[should_panic(expected = "State root mismatch")]
        fn mismatch() {
            let input = EvmInput::default();
            input.assert_state_root_coherency();
        }
    }

    mod assert_ancestors_coherency {
        use super::*;

        #[test]
        fn success() {
            let ancestor = EthBlockHeader::default();
            let input = EvmInput {
                ancestors: vec![Default::default()],
                header: Box::new(EthBlockHeader {
                    number: 1,
                    parent_hash: ancestor.hash_slow(),
                    ..Default::default()
                }),
                ..Default::default()
            };

            input.assert_ancestors_coherency();
        }

        #[test]
        #[should_panic(expected = "failed: Invalid chain: block 0 is not the parent of block 1")]
        fn mismatch() {
            let input = EvmInput {
                ancestors: vec![Default::default()],
                header: Box::new(EthBlockHeader {
                    number: 1,
                    ..Default::default()
                }),
                ..Default::default()
            };
            input.assert_ancestors_coherency();
        }
    }
}
