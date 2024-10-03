use std::{cell::RefCell, collections::HashMap, iter::once, rc::Rc};

use alloy_primitives::{Bytes, B256};
use block_header::EvmBlockHeader;
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
        let db = D::from(input.clone());
        EvmEnv::new(db, input.header)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MultiEvmInput {
    pub inputs: HashMap<ExecutionLocation, EvmInput>,
}

impl MultiEvmInput {
    pub fn new(inputs: HashMap<ExecutionLocation, EvmInput>) -> Self {
        Self { inputs }
    }

    pub fn from_entries<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (ExecutionLocation, EvmInput)>,
    {
        let inputs = iter.into_iter().collect();
        Self { inputs }
    }

    pub fn assert_coherency(&self) {
        for input in self.inputs.values() {
            input.assert_coherency();
        }
    }
}

impl FromIterator<(ExecutionLocation, EvmInput)> for MultiEvmInput {
    fn from_iter<I: IntoIterator<Item = (ExecutionLocation, EvmInput)>>(iter: I) -> Self {
        let inputs = iter.into_iter().collect();
        Self { inputs }
    }
}

impl IntoIterator for MultiEvmInput {
    type IntoIter = std::collections::hash_map::IntoIter<ExecutionLocation, EvmInput>;
    type Item = (ExecutionLocation, EvmInput);

    fn into_iter(self) -> Self::IntoIter {
        self.inputs.into_iter()
    }
}

impl<D> From<MultiEvmInput> for MultiEvmEnv<D>
where
    D: From<EvmInput>,
{
    fn from(input: MultiEvmInput) -> Self {
        RefCell::new(
            input
                .into_iter()
                .map(|(location, input)| {
                    let chain_spec = &location.chain_id.try_into().expect("cannot get chain spec");
                    (location, Rc::new(EvmEnv::from(input).with_chain_spec(chain_spec).unwrap()))
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
