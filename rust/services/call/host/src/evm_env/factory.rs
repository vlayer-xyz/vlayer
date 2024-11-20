use std::sync::Arc;

use alloy_primitives::address;
use call_engine::evm::env::{location::ExecutionLocation, EvmEnv, EvmEnvFactory};
use derive_new::new;
use provider::CachedMultiProvider;
use revm::db::CacheDB;

use crate::{Error, HostDb, ProofDb};

#[derive(new)]
pub(crate) struct HostEvmEnvFactory {
    providers: CachedMultiProvider,
}

fn seed_cache_db<D>(db: &mut CacheDB<D>) {
    db.insert_account_info(
        address!("1111111111111111111111111111111111111111"),
        Default::default(),
    );
}

impl EvmEnvFactory<HostDb> for HostEvmEnvFactory {
    fn create(
        &self,
        ExecutionLocation {
            block_number,
            chain_id,
        }: ExecutionLocation,
    ) -> anyhow::Result<EvmEnv<HostDb>> {
        let provider = self.providers.get(chain_id)?;
        let header = provider
            .get_block_header(block_number.into())
            .map_err(|err| Error::Provider(err.to_string()))?
            .ok_or(Error::BlockNotFound(block_number))?;

        let proof_db = ProofDb::new(Arc::clone(&provider), block_number);
        let mut db = CacheDB::new(proof_db);
        seed_cache_db(&mut db);

        let chain_spec = chain_id.try_into()?;
        let env = EvmEnv::new(db, header).with_chain_spec(&chain_spec)?;
        Ok(env)
    }
}
