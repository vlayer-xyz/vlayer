use std::{path::PathBuf, sync::Arc};

use alloy_chains::NamedChain::{Mainnet, OptimismSepolia};
use alloy_primitives::address;
use call_engine::{
    evm::env::{cached::CachedEvmEnv, location::ExecutionLocation},
    travel_call_executor::TravelCallExecutor,
};
use provider::{profiling, CachedMultiProvider, CachedProvider};

use crate::{
    evm_env::factory::HostEvmEnvFactory,
    test_harness::{
        contracts::{
            AVERAGE_BALANCE_OF_CALL, IERC20::balanceOfCall, SIMPLE_TIME_TRAVEL, USDT, USDT_BLOCK_NO,
        },
        rpc_snapshot_file,
    },
    Call,
};

fn profile(
    chain: &str,
    test_name: &str,
    location: ExecutionLocation,
    call: &Call,
) -> anyhow::Result<profiling::State> {
    let rpc_file = PathBuf::from(rpc_snapshot_file(chain, test_name));
    let provider = CachedProvider::from_file(&rpc_file)?;
    let profiling_provider = Arc::new(profiling::Provider::new(provider));
    let multi_provider =
        CachedMultiProvider::from_provider(location.chain_id, profiling_provider.clone());
    let envs = CachedEvmEnv::from_factory(HostEvmEnvFactory::new(multi_provider));

    let _ = TravelCallExecutor::new(&envs).call(call, location);

    Ok(profiling_provider.state())
}

#[tokio::test]
async fn usdt_erc20_balance_of() -> anyhow::Result<()> {
    let location: ExecutionLocation = (USDT_BLOCK_NO, Mainnet).into();
    let binance_8 = address!("F977814e90dA44bFA03b6295A0616a897441aceC");
    let call = Call::new(USDT, &balanceOfCall { account: binance_8 });

    let state = profile("mainnet", "usdt_erc20_balance_of", location, &call)?;

    assert_eq!(state.total_count(), 6);
    insta::with_settings!({sort_maps => true}, {
        insta::assert_yaml_snapshot!(state)
    });

    Ok(())
}

#[tokio::test]
async fn time_travel() -> anyhow::Result<()> {
    let location: ExecutionLocation = (20_064_547_u64, OptimismSepolia).into();
    let call = Call::new(SIMPLE_TIME_TRAVEL, &AVERAGE_BALANCE_OF_CALL);

    let state = profile("op_sepolia", "simple_time_travel", location, &call)?;

    assert_eq!(state.total_count(), 70);
    insta::with_settings!({sort_maps => true}, {
        insta::assert_yaml_snapshot!(state)
    });

    Ok(())
}
