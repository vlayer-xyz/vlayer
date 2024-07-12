use crate::host::{EthersClient, HostError};
use crate::provider::{EthFileProvider, EthersProvider, FileProvider, Provider};
use alloy_primitives::ChainId;
use derive_more::AsMut;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use vlayer_engine::ethereum::EthBlockHeader;

pub trait MultiProvider<P: Provider>
where
    Self: AsMut<HashMap<ChainId, Rc<P>>>,
{
    type Provider: Provider;
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

#[derive(AsMut)]
pub struct EthersMultiProvider {
    #[as_mut]
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

impl MultiProvider<EthersProvider<EthersClient>> for EthersMultiProvider {
    type Provider = EthersProvider<EthersClient>;
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

#[derive(AsMut)]
pub struct FileMultiProvider {
    #[as_mut]
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

impl MultiProvider<EthFileProvider> for FileMultiProvider {
    type Provider = EthFileProvider;
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

#[cfg(test)]
mod test {
    use vlayer_engine::config::MAINNET_ID;

    use crate::multiprovider::{EthersMultiProvider, MultiProvider};

    #[test]
    fn try_new_invalid_rpc_url() -> anyhow::Result<()> {
        let chain_id = MAINNET_ID;
        let rpc_urls = [(chain_id, "http://localhost:123".to_string())]
            .into_iter()
            .collect();
        let mut multi_provider = EthersMultiProvider::new(rpc_urls);
        let provider = multi_provider.get(chain_id)?;
        let res = provider.get_block_number();
        let error = res.unwrap_err();

        assert!(error.to_string().contains(
            "(http://localhost:123/): error trying to connect: tcp connect error: Connection refused"
        ));

        Ok(())
    }
}
