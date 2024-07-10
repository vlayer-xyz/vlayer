use crate::host::{EthersClient, HostError};
use crate::provider::EthersProvider;
use alloy_primitives::ChainId;
use ethers_providers::{Http, RetryClient};
use std::collections::HashMap;
use std::rc::Rc;

const MAX_RETRY: u32 = 3;
const INITIAL_BACKOFF: u64 = 500;

pub struct MultiProvider {
    rpc_urls: HashMap<ChainId, String>,
    providers: HashMap<ChainId, Rc<EthersProvider<ethers_providers::Provider<RetryClient<Http>>>>>,
}

impl MultiProvider {
    pub fn new(rpc_urls: HashMap<ChainId, String>) -> Self {
        MultiProvider {
            rpc_urls,
            providers: HashMap::new(),
        }
    }

    pub fn get_provider(
        &mut self,
        chain_id: ChainId,
    ) -> Result<Rc<EthersProvider<ethers_providers::Provider<RetryClient<Http>>>>, HostError> {
        if let Some(provider) = self.providers.get(&chain_id) {
            return Ok(Rc::clone(provider));
        }

        let url = self
            .rpc_urls
            .get(&chain_id)
            .ok_or(HostError::NoRpcUrl(chain_id))?;

        let client = EthersClient::new_client(url, MAX_RETRY, INITIAL_BACKOFF)?;

        let provider = EthersProvider::new(client);

        let rc_provider = Rc::new(provider);
        self.providers.insert(chain_id, Rc::clone(&rc_provider));

        Ok(rc_provider)
    }
}
