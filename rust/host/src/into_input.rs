use std::collections::HashMap;

use crate::{db::proof::ProofDb, provider::Provider};
use anyhow::{ensure, Ok};
use vlayer_engine::evm::block_header::EvmBlockHeader;
use vlayer_engine::evm::env::MultiEvmEnv;
use vlayer_engine::evm::input::{EvmInput, MultiEvmInput};

pub fn into_input<P: Provider>(
    db: &ProofDb<P>,
    header: Box<dyn EvmBlockHeader>,
) -> anyhow::Result<EvmInput> {
    let (state_trie, storage_tries) = db.prepare_state_storage_tries()?;
    ensure!(
        header.state_root() == &state_trie.hash_slow(),
        "root of the state trie does not match the header"
    );

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

pub fn into_multi_input<P: Provider>(
    envs: MultiEvmEnv<ProofDb<P>>,
) -> anyhow::Result<MultiEvmInput> {
    let mut inner = HashMap::new();
    for (location, env) in envs.0 {
        let header = env.header;
        let input = into_input(&env.db, header)?;
        inner.insert(location, input);
    }
    Ok(MultiEvmInput(inner))
}
