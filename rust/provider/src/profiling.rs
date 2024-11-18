use std::sync::atomic::{AtomicUsize, Ordering};

use alloy_primitives::{Address, BlockNumber, Bytes, StorageKey, StorageValue, TxNumber, U256};
use anyhow::Result;
use block_header::EvmBlockHeader;
use ethers_core::types::BlockNumber as BlockTag;

use super::{BlockingProvider, EIP1186Proof};

#[derive(Debug)]
pub struct ProfilingProvider {
    pub(super) inner: Box<dyn BlockingProvider>,
    pub(super) state: AtomicUsize,
}

impl ProfilingProvider {}

impl BlockingProvider for ProfilingProvider {
    fn get_balance(&self, address: Address, block: BlockNumber) -> Result<U256> {
        self.state.fetch_add(1, Ordering::SeqCst);
        self.inner.get_balance(address, block)
    }

    fn get_block_header(&self, block: BlockTag) -> Result<Option<Box<dyn EvmBlockHeader>>> {
        self.state.fetch_add(1, Ordering::SeqCst);
        self.inner.get_block_header(block)
    }

    fn get_code(&self, address: Address, block: BlockNumber) -> Result<Bytes> {
        self.state.fetch_add(1, Ordering::SeqCst);
        self.inner.get_code(address, block)
    }

    fn get_proof(
        &self,
        address: Address,
        storage_keys: Vec<StorageKey>,
        block: BlockNumber,
    ) -> Result<EIP1186Proof> {
        self.state.fetch_add(1, Ordering::SeqCst);
        self.inner.get_proof(address, storage_keys, block)
    }

    fn get_storage_at(
        &self,
        address: Address,
        key: StorageKey,
        block: BlockNumber,
    ) -> Result<StorageValue> {
        self.state.fetch_add(1, Ordering::SeqCst);
        self.inner.get_storage_at(address, key, block)
    }

    fn get_transaction_count(&self, address: Address, block: BlockNumber) -> Result<TxNumber> {
        self.state.fetch_add(1, Ordering::SeqCst);
        self.inner.get_transaction_count(address, block)
    }
}
