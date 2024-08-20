use alloy_primitives::{
    Address, BlockNumber, Bytes, ChainId, StorageKey, StorageValue, TxNumber, U256,
};
use ethers_core::types::BlockNumber as BlockTag;

use call_engine::block_header::EvmBlockHeader;
use call_engine::config::TESTING_CHAIN_ID;
use call_host::host::error::HostError;
use call_host::proof::EIP1186Proof;
use call_host::provider::factory::{EthersProviderFactory, ProviderFactory};
use call_host::provider::{BlockingProvider, EthersClient, EthersProvider};

use crate::providers::pending_state_provider::PendingStateProviderFactory;

pub struct TestProvider {
    provider: Box<
        dyn BlockingProvider<Error = <EthersProvider<EthersClient> as BlockingProvider>::Error>,
    >,
}

impl BlockingProvider for TestProvider {
    type Error = <EthersProvider<EthersClient> as BlockingProvider>::Error;

    fn get_balance(&self, address: Address, block: BlockNumber) -> Result<U256, Self::Error> {
        self.provider.get_balance(address, block)
    }

    fn get_block_header(
        &self,
        block: BlockTag,
    ) -> Result<Option<Box<dyn EvmBlockHeader>>, Self::Error> {
        self.provider.get_block_header(block)
    }

    fn get_code(&self, address: Address, block: BlockNumber) -> Result<Bytes, Self::Error> {
        self.provider.get_code(address, block)
    }

    fn get_proof(
        &self,
        address: Address,
        storage_keys: Vec<StorageKey>,
        block: BlockNumber,
    ) -> Result<EIP1186Proof, Self::Error> {
        self.provider.get_proof(address, storage_keys, block)
    }

    fn get_storage_at(
        &self,
        address: Address,
        key: StorageKey,
        block: BlockNumber,
    ) -> Result<StorageValue, Self::Error> {
        self.provider.get_storage_at(address, key, block)
    }

    fn get_transaction_count(
        &self,
        address: Address,
        block: BlockNumber,
    ) -> Result<TxNumber, Self::Error> {
        self.provider.get_transaction_count(address, block)
    }
}

pub struct TestProviderFactory {
    pending_state_provider_factory: PendingStateProviderFactory,
    ethers_provider_factory: EthersProviderFactory,
}

impl TestProviderFactory {
    pub fn new(
        pending_state_provider_factory: PendingStateProviderFactory,
        ethers_provider_factory: EthersProviderFactory,
    ) -> Self {
        TestProviderFactory {
            pending_state_provider_factory,
            ethers_provider_factory,
        }
    }
}

impl ProviderFactory<TestProvider> for TestProviderFactory {
    fn create(&self, chain_id: ChainId) -> Result<TestProvider, HostError> {
        if chain_id == TESTING_CHAIN_ID {
            let pending_state_provider = self.pending_state_provider_factory.create(chain_id)?;
            Ok(TestProvider {
                provider: Box::new(pending_state_provider),
            })
        } else {
            let ethers_provider = self.ethers_provider_factory.create(chain_id)?;
            Ok(TestProvider {
                provider: Box::new(ethers_provider),
            })
        }
    }
}
