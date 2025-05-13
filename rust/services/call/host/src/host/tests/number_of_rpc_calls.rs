use std::{path::PathBuf, sync::Arc};

use alloy_chains::NamedChain::{Mainnet, OptimismSepolia};
use alloy_primitives::address;
use call_common::ExecutionLocation;
use call_engine::{evm::env::cached::CachedEvmEnv, travel_call};
use call_rpc::rpc_cache_path;
use provider::{CachedMultiProvider, CachedProvider, profiling};

use crate::{
    Call,
    evm_env::factory::HostEvmEnvFactory,
    host::tests::call,
    test_harness::contracts::{
        time_travel::{self, AVERAGE_BALANCE_OF_CALL, SIMPLE_TIME_TRAVEL},
        usdt::{self, IERC20::balanceOfCall, USDT},
    },
};

fn profile(
    chain: &str,
    test_name: &str,
    location: ExecutionLocation,
    call: &Call,
) -> anyhow::Result<profiling::State> {
    let file_path = PathBuf::from(rpc_cache_path(chain, test_name));
    let provider = CachedProvider::from_file(&file_path)?;
    let profiling_provider = Arc::new(profiling::Provider::new(provider));
    let multi_provider =
        CachedMultiProvider::from_provider(location.chain_id, profiling_provider.clone());
    let envs = CachedEvmEnv::from_factory(HostEvmEnvFactory::new(multi_provider));

    let _ = travel_call::Executor::new(&envs, location, true).call(call);

    Ok(profiling_provider.state())
}

#[tokio::test]
async fn usdt_erc20_balance_of() -> anyhow::Result<()> {
    let location: ExecutionLocation = (Mainnet, usdt::BLOCK_NO).into();
    let binance_8 = address!("F977814e90dA44bFA03b6295A0616a897441aceC");
    let call = call(USDT, &balanceOfCall { account: binance_8 });

    let state = profile("mainnet", "usdt_erc20_balance_of", location, &call)?;

    assert_eq!(state.total_count(), 6);
    insta::with_settings!({sort_maps => true}, {
        insta::assert_yaml_snapshot!(state)
    });

    Ok(())
}

#[tokio::test]
async fn time_travel() -> anyhow::Result<()> {
    let location: ExecutionLocation = (OptimismSepolia, time_travel::BLOCK_NO).into();
    let call = call(SIMPLE_TIME_TRAVEL, &AVERAGE_BALANCE_OF_CALL);
    let state = profile("op_sepolia", "time_travel", location, &call)?;

    assert_eq!(state.total_count(), 79);
    insta::with_settings!({sort_maps => true}, {
        insta::assert_yaml_snapshot!(state)
    });

    Ok(())
}
