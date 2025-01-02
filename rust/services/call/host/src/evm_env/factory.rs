use std::sync::Arc;

use call_engine::{
    evm::env::{location::ExecutionLocation, EvmEnv, EvmEnvFactory},
    seed_cache_db_with_trusted_data,
};
use derive_new::new;
use provider::CachedMultiProvider;
use revm::db::CacheDB;

use crate::{Error, HostDb, ProofDb};

#[derive(new)]
pub(crate) struct HostEvmEnvFactory {
    providers: CachedMultiProvider,
}

impl EvmEnvFactory<HostDb> for HostEvmEnvFactory {
    fn create(
        &self,
        ExecutionLocation {
            block_number,
            chain_id,
        }: ExecutionLocation,
    ) -> anyhow::Result<EvmEnv<HostDb>> {
        let block_tag = block_number.into();
        let provider = self.providers.get(chain_id)?;
        let header = provider
            .get_block_header(block_tag)?
            .ok_or(Error::BlockNotFound(block_tag))?;

        let proof_db = ProofDb::new(Arc::clone(&provider), block_number);
        let mut db = CacheDB::new(proof_db);
        seed_cache_db_with_trusted_data(&mut db);

        let chain_spec = chain_id.try_into()?;
        let env = EvmEnv::new(db, header).with_chain_spec(&chain_spec)?;
        Ok(env)
    }
}
