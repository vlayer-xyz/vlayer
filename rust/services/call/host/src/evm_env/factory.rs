use std::sync::Arc;

use call_engine::evm::env::{location::ExecutionLocation, EvmEnv, EvmEnvFactory};
use provider::CachedMultiProvider;

use crate::{db::proof::ProofDb, host::error::HostError};

pub(crate) struct HostEvmEnvFactory {
    providers: CachedMultiProvider,
}

impl HostEvmEnvFactory {
    pub(crate) const fn new(providers: CachedMultiProvider) -> Self {
        HostEvmEnvFactory { providers }
    }
}

impl EvmEnvFactory<ProofDb> for HostEvmEnvFactory {
    fn create(
        &self,
        ExecutionLocation {
            block_number,
            chain_id,
        }: ExecutionLocation,
    ) -> anyhow::Result<EvmEnv<ProofDb>> {
        let provider = self.providers.get(chain_id)?;
        let header = provider
            .get_block_header(block_number.into())
            .map_err(|err| HostError::Provider(err.to_string()))?
            .ok_or(HostError::BlockNotFound(block_number))?;

        let db = ProofDb::new(Arc::clone(&provider), block_number);
        let chain_spec = chain_id.try_into()?;
        let env = EvmEnv::new(db, header).with_chain_spec(&chain_spec)?;
        Ok(env)
    }
}
