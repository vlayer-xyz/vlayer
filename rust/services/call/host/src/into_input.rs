use crate::db::proof::ProofDb;
use anyhow::anyhow;
use anyhow::{ensure, Ok};
use call_engine::block_header::EvmBlockHeader;
use call_engine::evm::env::cached::CachedEvmEnv;
use call_engine::evm::input::{EvmInput, MultiEvmInput};
use provider::BlockingProvider;
use std::rc::Rc;

fn into_input<P: BlockingProvider>(
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

pub(crate) fn into_multi_input<P: BlockingProvider>(
    envs: CachedEvmEnv<ProofDb<P>>,
) -> anyhow::Result<MultiEvmInput> {
    envs.into_inner()
        .into_iter()
        .map(|(location, env)| {
            let env = Rc::try_unwrap(env).map_err(|rc| {
                anyhow!(
                    "Can't unwrap EvmEnv Rc as it still has {} strong references",
                    Rc::strong_count(&rc)
                )
            })?;
            Ok((location, into_input(&env.db, env.header)?))
        })
        .collect()
}
