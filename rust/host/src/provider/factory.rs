use crate::host::{EthersClient, HostError};
use crate::provider::{EthFileProvider, EthersProvider, Provider};
use alloy_primitives::ChainId;
use std::collections::HashMap;
use std::path::PathBuf;

pub trait ProviderFactory<P>
where
    P: Provider,
{
    fn create(&self, chain_id: ChainId) -> Result<P, HostError>;
}

pub struct EthersProviderFactory {
    rpc_urls: HashMap<ChainId, String>,
}

const MAX_RETRY: u32 = 3;
const INITIAL_BACKOFF: u64 = 500;

impl EthersProviderFactory {
    pub fn new(rpc_urls: HashMap<ChainId, String>) -> Self {
        EthersProviderFactory { rpc_urls }
    }
}

impl ProviderFactory<EthersProvider<EthersClient>> for EthersProviderFactory {
    fn create(&self, chain_id: ChainId) -> Result<EthersProvider<EthersClient>, HostError> {
        let url = self
            .rpc_urls
            .get(&chain_id)
            .ok_or(HostError::NoRpcUrl(chain_id))?;

        let client = EthersClient::new_client(url, MAX_RETRY, INITIAL_BACKOFF)?;

        Ok(EthersProvider::new(client))
    }
}

pub struct FileProviderFactory {
    rpc_file_cache: HashMap<ChainId, String>,
}

impl FileProviderFactory {
    pub fn new(rpc_file_cache: HashMap<ChainId, String>) -> Self {
        FileProviderFactory { rpc_file_cache }
    }
}

impl ProviderFactory<EthFileProvider> for FileProviderFactory {
    fn create(&self, chain_id: ChainId) -> Result<EthFileProvider, HostError> {
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

        Ok(provider)
    }
}

#[cfg(test)]
mod test {
    use vlayer_engine::config::MAINNET_ID;

    use super::*;

    #[test]
    fn try_new_invalid_rpc_url() -> anyhow::Result<()> {
        let chain_id = MAINNET_ID;
        let rpc_urls = [(chain_id, "http://localhost:123".to_string())]
            .into_iter()
            .collect();
        let factory = EthersProviderFactory::new(rpc_urls);
        let provider = factory.create(chain_id)?;
        let res = provider.get_block_number();
        let error = res.unwrap_err();

        assert!(error.to_string().contains(
            "(http://localhost:123/): error trying to connect: tcp connect error: Connection refused"
        ));

        Ok(())
    }
}
