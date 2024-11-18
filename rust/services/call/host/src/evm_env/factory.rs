use std::sync::Arc;

use call_engine::evm::env::{location::ExecutionLocation, EvmEnv, EvmEnvFactory};
use derive_new::new;
use provider::CachedMultiProvider;

use crate::{Error, ProofDb};

#[derive(new)]
pub(crate) struct HostEvmEnvFactory {
    providers: CachedMultiProvider,
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
            .map_err(|err| Error::Provider(err.to_string()))?
            .ok_or(Error::BlockNotFound(block_number))?;

        let db = ProofDb::new(Arc::clone(&provider), block_number);
        let chain_spec = chain_id.try_into()?;
        let env = EvmEnv::new(db, header).with_chain_spec(&chain_spec)?;
        Ok(env)
    }
}
