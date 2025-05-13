use std::sync::Arc;

use block_header::EvmBlockHeader;
use call_db::{ProofDb, ProofDbError};
use call_engine::evm::{
    env::cached::CachedEvmEnv,
    input::{EvmInput, MultiEvmInput},
};
use common::Hashable;
use mpt::{EMPTY_ROOT_HASH, MerkleTrie, Node};
use thiserror::Error;

use crate::HostDb;

#[derive(Debug, Error)]
pub enum Error {
    #[error("ProofDB: {0}")]
    ProofDB(#[from] ProofDbError),
    #[error("State root mismatch")]
    StateRootMismatch,
    #[error("Can't unwrap EvmEnv Arc as it still has {0} strong references")]
    OutstandingStrongReferencesOnArcUnwrap(usize),
}
pub type Result<T> = std::result::Result<T, Error>;

fn into_input(db: &ProofDb, header: Box<dyn EvmBlockHeader>) -> Result<EvmInput> {
    let (mut state_trie, storage_tries) = db.prepare_state_storage_tries()?;
    let state_root = state_trie.hash_slow();
    // If the trie contains no nodes - we replace it with empty trie that has correct digest
    // SAFETY: You can't get any account/storage info from such a trie
    if state_root == EMPTY_ROOT_HASH {
        state_trie = MerkleTrie(Node::Digest(*header.state_root()));
    } else if header.state_root() != &state_root {
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

#[cfg(test)]
mod test {
    use std::{marker::PhantomData, sync::Arc};

    use alloy_primitives::B256;
    use block_header::EthBlockHeader;
    use call_db::ProofDb;
    use mpt::{MerkleTrie, Node};

    use super::into_input;

    #[test]
    fn into_input_empty_trie() {
        let provider = provider::never::NeverProvider(PhantomData);
        let db = ProofDb::new(Arc::new(provider), 0);
        let state_root = B256::with_last_byte(1);
        let header = EthBlockHeader {
            state_root,
            ..Default::default()
        };
        let input = into_input(&db, Box::new(header)).unwrap();
        assert_eq!(input.state_trie, MerkleTrie(Node::Digest(state_root)));
    }
}
