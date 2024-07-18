use crate::{db::proof::ProofDb, provider::Provider};
use anyhow::{ensure, Ok};
use vlayer_engine::block_header::evm::EvmBlockHeader;
use vlayer_engine::evm::env::MultiEvmEnv;
use vlayer_engine::evm::input::{EvmInput, MultiEvmInput};

pub fn into_input<P: Provider>(
    db: &ProofDb<P>,
    header: P::Header,
) -> anyhow::Result<EvmInput<P::Header>> {
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
    envs: MultiEvmEnv<ProofDb<P>, P::Header>,
) -> anyhow::Result<MultiEvmInput<P::Header>> {
    envs.into_iter()
        .map(|(location, env)| Ok((location, into_input(&env.db, env.header)?)))
        .collect()
}
