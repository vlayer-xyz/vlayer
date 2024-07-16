use std::rc::Rc;

use vlayer_engine::evm::env::{EvmEnv, ExecutionLocation};

use crate::{db::proof::ProofDb, host::HostError, provider::Provider};

pub struct EvmEnvFactory<P> {
    provider: Rc<P>,
}

impl<P> EvmEnvFactory<P>
where
    P: Provider,
{
    pub fn new(provider: Rc<P>) -> Self {
        EvmEnvFactory { provider }
    }

    pub fn create(
        &self,
        ExecutionLocation {
            block_number,
            chain_id,
        }: ExecutionLocation,
    ) -> Result<EvmEnv<ProofDb<P>, P::Header>, HostError> {
        let header = self
            .provider
            .get_block_header(block_number)
            .map_err(|err| HostError::Provider(err.to_string()))?
            .ok_or(HostError::BlockNotFound(block_number))?;

        let db = ProofDb::new(Rc::clone(&self.provider), block_number);
        let chain_spec = chain_id.try_into()?;
        let mut env = EvmEnv::new(db, header);
        env.with_chain_spec(&chain_spec)?;
        Ok(env)
    }
}
