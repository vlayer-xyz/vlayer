use std::sync::{Arc, RwLock};

use call_common::ExecutionLocation;
use call_engine::{
    evm::{
        env::{EvmEnv, cached::MultiEvmEnv},
        input::{EvmInput, MultiEvmInput},
    },
    seed_cache_db_with_trusted_data,
};
use revm::db::CacheDB;

use crate::db::{GuestDb, wrap_state::WrapStateDb};

#[allow(clippy::expect_used)]
fn create_env(location: ExecutionLocation, input: EvmInput) -> Arc<EvmEnv<GuestDb>> {
    let chain_spec = &location.chain_id.try_into().expect("cannot get chain spec");
    let header = input.header.clone();

    let mut db = CacheDB::new(WrapStateDb::from(input));
    seed_cache_db_with_trusted_data(&mut db);

    let env = EvmEnv::new(db, header)
        .with_chain_spec(chain_spec)
        .expect("failed to set chain spec");

    #[allow(clippy::arc_with_non_send_sync)]
    Arc::new(env)
}

pub fn create_envs_from_input(input: MultiEvmInput) -> MultiEvmEnv<GuestDb> {
    let env_map = input
        .into_iter()
        .map(|(location, input)| (location, create_env(location, input)))
        .collect();
    RwLock::new(env_map)
}

#[cfg(test)]
mod create_env {
    use super::*;

    #[test]
    #[should_panic(expected = "cannot get chain spec: UnsupportedChainId(0)")]
    fn panics_with_invalid_chain_spec() {
        let location = ExecutionLocation::default();
        let input = EvmInput::default();
        create_env(location, input);
    }

    #[test]
    fn success() {
        let location = ExecutionLocation::new(1, 0);
        let input = EvmInput::default();
        let env = create_env(location, input);

        assert_eq!(env.header().number(), 0);
    }
}
