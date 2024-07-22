use crate::{db::proof::ProofDb, provider::Provider};
use anyhow::anyhow;
use anyhow::{ensure, Ok};
use std::rc::Rc;
use vlayer_engine::block_header::EvmBlockHeader;
use vlayer_engine::evm::env::MultiEvmEnv;
use vlayer_engine::evm::input::{EvmInput, MultiEvmInput};

pub fn into_input<P: Provider>(
    db: ProofDb<P>,
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
        .map(|(location, env)| {
            let env = Rc::try_unwrap(env).map_err(|rc| {
                anyhow!(
                    "Can't unwrap EvmEnv Rc as it still has {} strong references",
                    Rc::strong_count(&rc)
                )
            })?;
            Ok((location, into_input(env.db, env.header)?))
        })
        .collect()
}
