use std::sync::atomic::{AtomicUsize, Ordering};

use alloy_primitives::{Address, BlockNumber, Bytes, StorageKey, StorageValue, TxNumber, U256};
use anyhow::Result;
use block_header::EvmBlockHeader;
use ethers_core::types::BlockNumber as BlockTag;

use super::{BlockingProvider, EIP1186Proof};

#[derive(Debug)]
pub struct ProfilingProvider {
    inner: Box<dyn BlockingProvider>,
    state: AtomicUsize,
}

impl ProfilingProvider {
    pub fn new(inner: impl BlockingProvider + 'static) -> Self {
        Self {
            inner: Box::new(inner),
            state: AtomicUsize::new(0),
        }
    }

    pub fn increment(&self) {
        self.state.fetch_add(1, Ordering::SeqCst);
    }

    pub fn call_count(&self) -> usize {
        self.state.load(Ordering::SeqCst)
    }
}

impl BlockingProvider for ProfilingProvider {
    fn get_balance(&self, address: Address, block: BlockNumber) -> Result<U256> {
        self.increment();
        self.inner.get_balance(address, block)
    }

    fn get_block_header(&self, block: BlockTag) -> Result<Option<Box<dyn EvmBlockHeader>>> {
        self.increment();
        self.inner.get_block_header(block)
    }

    fn get_code(&self, address: Address, block: BlockNumber) -> Result<Bytes> {
        self.increment();
        self.inner.get_code(address, block)
    }

    fn get_proof(
        &self,
        address: Address,
        storage_keys: Vec<StorageKey>,
        block: BlockNumber,
    ) -> Result<EIP1186Proof> {
        self.increment();
        self.inner.get_proof(address, storage_keys, block)
    }

    fn get_storage_at(
        &self,
        address: Address,
        key: StorageKey,
        block: BlockNumber,
    ) -> Result<StorageValue> {
        self.increment();
        self.inner.get_storage_at(address, key, block)
    }

    fn get_transaction_count(&self, address: Address, block: BlockNumber) -> Result<TxNumber> {
        self.increment();
        self.inner.get_transaction_count(address, block)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::default::DefaultProvider;

    #[test]
    fn test_profiling() -> Result<()> {
        let provider = ProfilingProvider::new(DefaultProvider);

        assert_eq!(provider.call_count(), 0);

        provider.get_balance(Default::default(), Default::default())?;
        assert_eq!(provider.call_count(), 1);

        provider.get_block_header(Default::default())?;
        assert_eq!(provider.call_count(), 2);

        provider.get_code(Default::default(), Default::default())?;
        assert_eq!(provider.call_count(), 3);

        provider.get_proof(Default::default(), Default::default(), Default::default())?;
        assert_eq!(provider.call_count(), 4);

        provider.get_storage_at(Default::default(), Default::default(), Default::default())?;
        assert_eq!(provider.call_count(), 5);

        provider.get_transaction_count(Default::default(), Default::default())?;
        assert_eq!(provider.call_count(), 6);

        Ok(())
    }
}
