use std::{
    collections::HashMap,
    iter::once,
    sync::{Arc, RwLock},
};

use alloy_primitives::{BlockHash, BlockNumber, Bytes, ChainId, B256};
use block_header::EvmBlockHeader;
use chain_types::ChainProof;
use derive_more::{From, Into, IntoIterator};
use derive_new::new;
use mpt::MerkleTrie;
use serde::{Deserialize, Serialize};
use tracing::debug;

use super::env::{cached::MultiEvmEnv, location::ExecutionLocation, EvmEnv};

/// The serializable input to derive and validate a [EvmEnv].
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EvmInput {
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

impl<D> From<EvmInput> for EvmEnv<D>
where
    D: From<EvmInput>,
{
    fn from(input: EvmInput) -> Self {
        let header = input.header.clone();
        EvmEnv::new(D::from(input), header)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, From, Into, IntoIterator, new)]
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

    pub fn assert_coherency(
        &self,
        chain_proofs: HashMap<ChainId, ChainProof>,
        verify_chain_proofs: bool,
    ) {
        self.inputs.values().for_each(EvmInput::assert_coherency);
        if verify_chain_proofs {
            self.assert_chain_coherence(chain_proofs);
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    fn assert_chain_coherence(&self, chain_proofs: HashMap<ChainId, ChainProof>) {
        for (chain_id, blocks) in self.group_blocks_by_chain() {
            let chain_proof = chain_proofs.get(&chain_id).expect("chain proof not found");
            for (block_number, block_hash) in blocks {
                assert_eq!(
                    chain_proof
                        .block_trie
                        .get(block_number)
                        .expect("block hash not found"),
                    block_hash,
                    "block hash mismatch"
                );
            }
        }
    }

    pub fn group_blocks_by_chain(&self) -> HashMap<ChainId, HashMap<BlockNumber, BlockHash>> {
        self.inputs
            .iter()
            .fold(HashMap::new(), |mut acc, (loc, evm_input)| {
                acc.entry(loc.chain_id)
                    .or_default()
                    .insert(loc.block_number, evm_input.header.hash_slow());
                acc
            })
    }
}

impl FromIterator<(ExecutionLocation, EvmInput)> for MultiEvmInput {
    fn from_iter<I: IntoIterator<Item = (ExecutionLocation, EvmInput)>>(iter: I) -> Self {
        let inputs = iter.into_iter().collect();
        Self { inputs }
    }
}

impl<D> From<MultiEvmInput> for MultiEvmEnv<D>
where
    D: From<EvmInput>,
{
    fn from(input: MultiEvmInput) -> Self {
        RwLock::new(
            input
                .into_iter()
                .map(|(location, input)| {
                    let chain_spec = &location.chain_id.try_into().expect("cannot get chain spec");
                    (location, Arc::new(EvmEnv::from(input).with_chain_spec(chain_spec).unwrap()))
                })
                .collect(),
        )
    }
}

#[cfg(test)]
mod test {
    use block_header::{EthBlockHeader, Hashable};
    use mpt::{MerkleTrie, EMPTY_ROOT_HASH};

    use super::EvmInput;

    impl Default for EvmInput {
        fn default() -> Self {
            Self {
                header: Box::new(EthBlockHeader::default()),
                ancestors: vec![],
                state_trie: MerkleTrie::default(),
                storage_tries: Vec::default(),
                contracts: Vec::default(),
            }
        }
    }

    mod block_hashes {
        use super::*;

        #[test]
        fn success() {
            let ancestor: EthBlockHeader = EthBlockHeader::default();
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
            let ancestor: EthBlockHeader = Default::default();
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
