use std::{collections::HashMap, sync::RwLock};

use alloy_primitives::{Address, BlockNumber, Bytes, StorageKey, StorageValue, TxNumber, U256};
use block_header::EvmBlockHeader;
use ethers_core::types::BlockNumber as BlockTag;
use serde::{Deserialize, Serialize};

use super::{BlockingProvider, EIP1186Proof, Result};

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct State {
    pub latest_block: u64,
    pub header: HashMap<BlockTag, u64>,
    pub balance: HashMap<BlockNumber, HashMap<Address, u64>>,
    pub code: HashMap<BlockNumber, HashMap<Address, u64>>,
    pub nonce: HashMap<BlockNumber, HashMap<Address, u64>>,
    pub proof: HashMap<BlockNumber, HashMap<Address, u64>>,
    pub storage: HashMap<BlockNumber, HashMap<Address, HashMap<StorageKey, u64>>>,
}

fn flatten<K>(map: &HashMap<K, u64>) -> impl Iterator<Item = &u64> {
    map.values()
}

fn flatten2<K1, K2>(map: &HashMap<K1, HashMap<K2, u64>>) -> impl Iterator<Item = &u64> {
    map.values().flat_map(flatten)
}

fn flatten3<K1, K2, K3>(
    map: &HashMap<K1, HashMap<K2, HashMap<K3, u64>>>,
) -> impl Iterator<Item = &u64> {
    map.values().flat_map(flatten2)
}

impl State {
    pub fn total_count(&self) -> u64 {
        flatten(&self.header)
            .chain(flatten2(&self.balance))
            .chain(flatten2(&self.code))
            .chain(flatten2(&self.nonce))
            .chain(flatten2(&self.proof))
            .chain(flatten3(&self.storage))
            .sum::<u64>()
            + self.latest_block
    }
}

#[derive(Debug)]
pub struct Provider {
    inner: Box<dyn BlockingProvider>,
    state: RwLock<State>,
}

impl Provider {
    pub fn new(inner: impl BlockingProvider + 'static) -> Self {
        Self {
            inner: Box::new(inner),
            state: Default::default(),
        }
    }

    #[allow(clippy::expect_used)]
    pub fn state(&self) -> State {
        self.state.read().expect("poisoned lock").clone()
    }
}

macro_rules! inc {
    ($val:expr, $field:ident, $($key:expr),+) => {{
        let val = &mut $val.write().expect("poisoned lock").$field;
        $(
            let val = val.entry($key).or_default();
        )+
        *val += 1;
    }};
}

impl BlockingProvider for Provider {
    fn get_balance(&self, address: Address, block: BlockNumber) -> Result<U256> {
        inc!(self.state, balance, block, address);
        self.inner.get_balance(address, block)
    }

    fn get_block_header(&self, block: BlockTag) -> Result<Option<Box<dyn EvmBlockHeader>>> {
        inc!(self.state, header, block);
        self.inner.get_block_header(block)
    }

    fn get_code(&self, address: Address, block: BlockNumber) -> Result<Bytes> {
        inc!(self.state, code, block, address);
        self.inner.get_code(address, block)
    }

    fn get_proof(
        &self,
        address: Address,
        storage_keys: Vec<StorageKey>,
        block: BlockNumber,
    ) -> Result<EIP1186Proof> {
        inc!(self.state, proof, block, address);
        self.inner.get_proof(address, storage_keys, block)
    }

    fn get_storage_at(
        &self,
        address: Address,
        key: StorageKey,
        block: BlockNumber,
    ) -> Result<StorageValue> {
        inc!(self.state, storage, block, address, key);
        self.inner.get_storage_at(address, key, block)
    }

    fn get_transaction_count(&self, address: Address, block: BlockNumber) -> Result<TxNumber> {
        inc!(self.state, nonce, block, address);
        self.inner.get_transaction_count(address, block)
    }

    #[allow(clippy::expect_used)]
    fn get_latest_block_number(&self) -> Result<BlockNumber> {
        self.state.write().expect("poisoned lock").latest_block += 1;
        self.inner.get_latest_block_number()
    }
}

#[cfg(test)]
mod tests {
    use maplit::hashmap as m;

    use super::*;
    use crate::default::DefaultProvider;

    #[test]
    fn test_profiling() -> Result<()> {
        let provider = Provider::new(DefaultProvider);

        provider.get_latest_block_number()?;
        provider.get_balance(Default::default(), Default::default())?;
        provider.get_block_header(Default::default())?;
        provider.get_code(Default::default(), Default::default())?;
        provider.get_proof(Default::default(), Default::default(), Default::default())?;
        provider.get_storage_at(Default::default(), Default::default(), Default::default())?;
        provider.get_transaction_count(Default::default(), Default::default())?;

        let expected_state = State {
            latest_block: 1,
            header: m! { BlockTag::Latest => 1 },
            balance: m! { 0 => m! { Address::ZERO => 1 } },
            code: m! { 0 => m! { Address::ZERO => 1 } },
            nonce: m! { 0 => m! { Address::ZERO => 1 } },
            proof: m! { 0 => m! { Address::ZERO => 1 } },
            storage: m! { 0 => m! { Address::ZERO => m! { StorageKey::ZERO => 1 } } },
        };
        assert_eq!(provider.state(), expected_state);

        Ok(())
    }
}
