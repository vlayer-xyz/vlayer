use crate::host::{EthersClient, HostError};
use crate::provider::{EthersProvider, Provider};
use alloy_primitives::ChainId;
use std::collections::HashMap;
use std::rc::Rc;

pub trait MultiProvider<P: Provider>
where
    Self: AsMut<HashMap<ChainId, Rc<P>>>,
{
    fn create_provider(&mut self, chain_id: ChainId) -> Result<Rc<P>, HostError>;
    fn get(&mut self, chain_id: ChainId) -> Result<Rc<P>, HostError> {
        if let Some(provider) = self.as_mut().get(&chain_id) {
            return Ok(Rc::clone(provider));
        }

        let provider = self.create_provider(chain_id)?;

        self.as_mut().insert(chain_id, Rc::clone(&provider));
        Ok(provider)
    }
}

const MAX_RETRY: u32 = 3;
const INITIAL_BACKOFF: u64 = 500;

pub struct EthersMultiProvider {
    providers: HashMap<ChainId, Rc<EthersProvider<EthersClient>>>,
    rpc_urls: HashMap<ChainId, String>,
}

impl EthersMultiProvider {
    pub fn new(rpc_urls: HashMap<ChainId, String>) -> Self {
        EthersMultiProvider {
            providers: HashMap::new(),
            rpc_urls,
        }
    }
}

impl AsMut<HashMap<ChainId, Rc<EthersProvider<EthersClient>>>> for EthersMultiProvider {
    fn as_mut(&mut self) -> &mut HashMap<ChainId, Rc<EthersProvider<EthersClient>>> {
        &mut self.providers
    }
}

impl MultiProvider<EthersProvider<EthersClient>> for EthersMultiProvider {
    fn create_provider(
        &mut self,
        chain_id: ChainId,
    ) -> Result<Rc<EthersProvider<EthersClient>>, HostError> {
        let url = self
            .rpc_urls
            .get(&chain_id)
            .ok_or(HostError::NoRpcUrl(chain_id))?;

        let client = EthersClient::new_client(url, MAX_RETRY, INITIAL_BACKOFF)?;

        Ok(Rc::new(EthersProvider::new(client)))
    }
}
