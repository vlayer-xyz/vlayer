use std::sync::Arc;

use call_common::RevmDB;
use call_precompiles::precompiles as generate_precompiles;
use revm::{
    Evm, Handler, db::WrapDatabaseRef, inspector_handle_register, precompile::PrecompileWithAddress,
};

use super::inspector::Inspector;
use crate::{Call, evm::env::EvmEnv};

pub fn build_evm<'inspector, 'envs, D: RevmDB>(
    env: &'envs EvmEnv<D>,
    tx: &Call,
    inspector: Inspector<'inspector, D>,
    is_vlayer_test: bool,
) -> Evm<'inspector, Inspector<'inspector, D>, WrapDatabaseRef<&'envs D>> {
    let precompiles_handle_register = move |handler: &mut Handler<_, _, _>| {
        let mut precompiles = handler.pre_execution.load_precompiles();
        precompiles.extend(
            generate_precompiles(is_vlayer_test)
                .into_iter()
                .map(PrecompileWithAddress::from),
        );
        handler.pre_execution.load_precompiles = Arc::new(move || precompiles.clone());
    };

    let mut evm = Evm::builder()
        .with_ref_db(&env.db)
        .with_external_context(inspector)
        .with_cfg_env_with_handler_cfg(env.cfg_env.clone())
        .with_tx_env(tx.clone().into())
        .append_handler_register_box(Box::new(precompiles_handle_register))
        .append_handler_register(inspector_handle_register)
        .modify_block_env(|blk_env| env.header.fill_block_env(blk_env))
        .build();

    if evm.handler_cfg().is_optimism() {
        preload_l1_block_info(&mut evm);
    }

    evm
}

// EVM does it on itself in transaction validation, but we use transact_preverified so we need to do it manually.
fn preload_l1_block_info<D: RevmDB>(evm: &mut Evm<'_, Inspector<'_, D>, WrapDatabaseRef<&D>>) {
    let spec_id = evm.spec_id();
    let l1_block_info = revm::optimism::L1BlockInfo::try_fetch(evm.db_mut(), spec_id).expect(
        "Failed to fetch L1 block info. This should not happen as we preload all necesary data in seed_cache_db_with_trusted_data",
    );
    evm.context.evm.l1_block_info = Some(l1_block_info);
}
