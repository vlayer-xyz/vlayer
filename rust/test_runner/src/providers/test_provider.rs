use std::collections::HashMap;

use alloy_primitives::{
    Address, BlockNumber, Bytes, ChainId, StorageKey, StorageValue, TxNumber, U256,
};
use block_header::EvmBlockHeader;
use chain::{CHAIN_NAMES, TEST_CHAIN_ID};
use ethers_core::types::BlockNumber as BlockTag;
use foundry_config::RpcEndpoints;
use provider::{
    BlockingProvider, EIP1186Proof, EthersProviderFactory, ProviderFactory, ProviderFactoryError,
};

use crate::providers::pending_state_provider::PendingStateProviderFactory;

#[derive(Debug)]
pub struct TestProvider {
    provider: Box<dyn BlockingProvider>,
}

impl BlockingProvider for TestProvider {
    fn get_balance(&self, address: Address, block: BlockNumber) -> anyhow::Result<U256> {
        self.provider.get_balance(address, block)
    }

    fn get_block_header(&self, block: BlockTag) -> anyhow::Result<Option<Box<dyn EvmBlockHeader>>> {
        self.provider.get_block_header(block)
    }

    fn get_code(&self, address: Address, block: BlockNumber) -> anyhow::Result<Bytes> {
        self.provider.get_code(address, block)
    }

    fn get_proof(
        &self,
        address: Address,
        storage_keys: Vec<StorageKey>,
        block: BlockNumber,
    ) -> anyhow::Result<EIP1186Proof> {
        self.provider.get_proof(address, storage_keys, block)
    }

    fn get_storage_at(
        &self,
        address: Address,
        key: StorageKey,
        block: BlockNumber,
    ) -> anyhow::Result<StorageValue> {
        self.provider.get_storage_at(address, key, block)
    }

    fn get_transaction_count(
        &self,
        address: Address,
        block: BlockNumber,
    ) -> anyhow::Result<TxNumber> {
        self.provider.get_transaction_count(address, block)
    }
}

pub struct TestProviderFactory {
    pending_state_provider_factory: PendingStateProviderFactory,
    rpc_endpoints: RpcEndpoints,
}

impl TestProviderFactory {
    pub const fn new(
        pending_state_provider_factory: PendingStateProviderFactory,
        rpc_endpoints: RpcEndpoints,
    ) -> Self {
        TestProviderFactory {
            pending_state_provider_factory,
            rpc_endpoints,
        }
    }

    fn get_rpc_url(&self, chain_id: ChainId) -> HashMap<ChainId, String> {
        for (id, rpc_endpoint) in self.rpc_endpoints.iter() {
            if CHAIN_NAMES.get(id) == Some(&chain_id) {
                return HashMap::from([(chain_id, rpc_endpoint.endpoint.as_url().unwrap().into())]);
            }
        }
        Default::default()
    }
}

impl ProviderFactory for TestProviderFactory {
    fn create(&self, chain_id: ChainId) -> Result<Box<dyn BlockingProvider>, ProviderFactoryError> {
        if chain_id == TEST_CHAIN_ID {
            let pending_state_provider = self.pending_state_provider_factory.create(chain_id)?;
            Ok(Box::new(TestProvider {
                provider: pending_state_provider,
            }))
        } else {
            let ethers_provider_factory = EthersProviderFactory::new(self.get_rpc_url(chain_id));
            let ethers_provider = ethers_provider_factory.create(chain_id)?;
            Ok(Box::new(TestProvider {
                provider: ethers_provider,
            }))
        }
    }
}
