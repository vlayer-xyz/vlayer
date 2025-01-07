use std::sync::Arc;

use block_header::EvmBlockHeader;
use call_engine::evm::{
    env::cached::CachedEvmEnv,
    input::{EvmInput, MultiEvmInput},
};
use common::Hashable;
use thiserror::Error;

use crate::db::{
    proof::{self, ProofDb},
    HostDb,
};

#[derive(Debug, Error)]
pub enum Error {
    #[error("ProofDB: {0}")]
    ProofDB(#[from] proof::Error),
    #[error("State root mismatch")]
    StateRootMismatch,
    #[error("Can't unwrap EvmEnv Arc as it still has {0} strong references")]
    OutstandingStrongReferencesOnArcUnwrap(usize),
}
pub type Result<T> = std::result::Result<T, Error>;

fn into_input(db: &ProofDb, header: Box<dyn EvmBlockHeader>) -> Result<EvmInput> {
    let (state_trie, storage_tries) = db.prepare_state_storage_tries()?;
    if header.state_root() != &state_trie.hash_slow() {
        return Err(Error::StateRootMismatch);
    }

    let evm_input = EvmInput {
        header,
        state_trie,
        storage_tries,
        contracts: db.contracts(),
        ancestors: db.fetch_ancestors()?,
    };
    evm_input.print_sizes();

    Ok(evm_input)
}

pub(crate) fn into_multi_input(envs: CachedEvmEnv<HostDb>) -> Result<MultiEvmInput> {
    envs.into_inner()
        .into_iter()
        .map(|(location, env)| {
            let env = Arc::try_unwrap(env).map_err(|rc| {
                Error::OutstandingStrongReferencesOnArcUnwrap(Arc::strong_count(&rc))
            })?;
            Ok((location, into_input(&env.db.db, env.header)?))
        })
        .collect()
}
