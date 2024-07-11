use crate::host::{EthersClient, HostError};
use crate::provider::{EthFileProvider, EthersProvider, FileProvider, Provider};
use alloy_primitives::ChainId;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use vlayer_engine::ethereum::EthBlockHeader;

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

pub struct FileMultiProvider {
    providers: HashMap<ChainId, Rc<FileProvider<EthBlockHeader>>>,
    rpc_file_cache: HashMap<ChainId, String>,
}

impl FileMultiProvider {
    pub fn new(rpc_file_cache: HashMap<ChainId, String>) -> Self {
        FileMultiProvider {
            providers: HashMap::new(),
            rpc_file_cache,
        }
    }
}

impl AsMut<HashMap<ChainId, Rc<EthFileProvider>>> for FileMultiProvider {
    fn as_mut(&mut self) -> &mut HashMap<ChainId, Rc<EthFileProvider>> {
        &mut self.providers
    }
}

impl MultiProvider<EthFileProvider> for FileMultiProvider {
    fn create_provider(&mut self, chain_id: ChainId) -> Result<Rc<EthFileProvider>, HostError> {
        let file_path = self
            .rpc_file_cache
            .get(&chain_id)
            .ok_or_else(|| HostError::NoRpcCache(chain_id))?;

        let path_buf = PathBuf::from(file_path);
        let provider = EthFileProvider::from_file(&path_buf).map_err(|err| {
            HostError::Provider(format!(
                "Error creating provider for chain ID {}: {}",
                chain_id, err
            ))
        })?;

        Ok(Rc::new(provider))
    }
}
