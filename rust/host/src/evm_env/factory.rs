use std::rc::Rc;

use vlayer_engine::evm::env::{location::ExecutionLocation, EvmEnv, EvmEnvFactory};

use crate::{
    db::proof::ProofDb,
    host::error::HostError,
    provider::{multi::CachedMultiProvider, Provider},
};

pub struct HostEvmEnvFactory<P> {
    providers: CachedMultiProvider<P>,
}

impl<P> HostEvmEnvFactory<P>
where
    P: Provider,
{
    pub fn new(providers: CachedMultiProvider<P>) -> Self {
        HostEvmEnvFactory { providers }
    }
}

impl<P> EvmEnvFactory<ProofDb<P>> for HostEvmEnvFactory<P>
where
    P: Provider,
{
    fn create(
        &self,
        ExecutionLocation {
            block_number,
            chain_id,
        }: ExecutionLocation,
    ) -> anyhow::Result<EvmEnv<ProofDb<P>>> {
        let provider = self.providers.get(chain_id)?;
        let header = provider
            .get_block_header(block_number)
            .map_err(|err| HostError::Provider(err.to_string()))?
            .ok_or(HostError::BlockNotFound(block_number))?;

        let db = ProofDb::new(Rc::clone(&provider), block_number);
        let chain_spec = chain_id.try_into()?;
        let env = EvmEnv::new(db, header).with_chain_spec(&chain_spec)?;
        Ok(env)
    }
}
