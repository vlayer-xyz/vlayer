use crate::host::{EthersClient, HostError};
use crate::provider::EthersProvider;
use alloy_primitives::ChainId;
use std::collections::HashMap;
use std::rc::Rc;

const MAX_RETRY: u32 = 3;
const INITIAL_BACKOFF: u64 = 500;

pub struct MultiProvider {
    rpc_urls: HashMap<ChainId, String>,
    providers: HashMap<ChainId, Rc<EthersProvider<EthersClient>>>,
}

impl MultiProvider {
    pub fn new(rpc_urls: HashMap<ChainId, String>) -> Self {
        MultiProvider {
            rpc_urls,
            providers: HashMap::new(),
        }
    }

    fn get_rpc_url(&self, chain_id: ChainId) -> Result<&String, HostError> {
        self.rpc_urls
            .get(&chain_id)
            .ok_or(HostError::NoRpcUrl(chain_id))
    }

    pub fn get_provider(
        &mut self,
        chain_id: ChainId,
    ) -> Result<Rc<EthersProvider<EthersClient>>, HostError> {
        if let Some(provider) = self.providers.get(&chain_id) {
            return Ok(provider.clone());
        }

        let url = self.get_rpc_url(chain_id)?;

        let client = EthersClient::new_client(url, MAX_RETRY, INITIAL_BACKOFF)?;

        let provider = EthersProvider::new(client);

        let rc_provider = Rc::new(provider);
        self.providers.insert(chain_id, rc_provider.clone());

        Ok(rc_provider)
    }
}
