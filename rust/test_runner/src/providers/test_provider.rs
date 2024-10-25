use std::collections::HashMap;

use alloy_primitives::ChainId;
use chain::{CHAIN_NAMES, TEST_CHAIN_ID};
use foundry_config::RpcEndpoints;
use provider::{BlockingProvider, EthersProviderFactory, ProviderFactory, ProviderFactoryError};

use crate::providers::pending_state_provider::PendingStateProviderFactory;

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
        let provider = if chain_id == TEST_CHAIN_ID {
            self.pending_state_provider_factory.create(chain_id)?
        } else {
            let ethers_provider_factory = EthersProviderFactory::new(self.get_rpc_url(chain_id));
            ethers_provider_factory.create(chain_id)?
        };
        Ok(provider)
    }
}
