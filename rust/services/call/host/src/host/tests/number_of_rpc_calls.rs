use std::{path::PathBuf, sync::Arc};

use alloy_chains::NamedChain::OptimismSepolia;
use call_engine::{
    evm::env::{cached::CachedEvmEnv, location::ExecutionLocation},
    travel_call_executor::TravelCallExecutor,
};
use provider::{profiling, CachedMultiProvider, CachedProvider};

use crate::{
    evm_env::factory::HostEvmEnvFactory,
    test_harness::{
        contracts::{AVERAGE_BALANCE_OF_CALL, SIMPLE_TIME_TRAVEL},
        rpc_snapshot_file,
    },
    Call,
};

#[tokio::test]
async fn time_travel() -> anyhow::Result<()> {
    let location: ExecutionLocation = (20_064_547_u64, OptimismSepolia).into();
    let call = Call::new(SIMPLE_TIME_TRAVEL, &AVERAGE_BALANCE_OF_CALL);

    let rpc_file = PathBuf::from(rpc_snapshot_file("op_sepolia", "simple_time_travel"));
    let provider = CachedProvider::from_file(&rpc_file)?;
    let profiling_provider = Arc::new(profiling::Provider::new(provider));
    let multi_provider =
        CachedMultiProvider::from_provider(location.chain_id, profiling_provider.clone());
    let envs = CachedEvmEnv::from_factory(HostEvmEnvFactory::new(multi_provider));

    let _ = TravelCallExecutor::new(&envs).call(&call, location);

    assert_eq!(profiling_provider.state().total_count(), 88);
    insta::with_settings!({sort_maps => true}, {
        insta::assert_yaml_snapshot!(profiling_provider.state())
    });

    Ok(())
}
