use std::sync::Arc;

use alloy_chains::{Chain, NamedChain};
use call_engine::{
    evm::env::{cached::CachedEvmEnv, location::ExecutionLocation},
    travel_call_executor::TravelCallExecutor,
};
use provider::{CachedMultiProvider, CachedProviderFactory, ProfilingProvider, ProviderFactory};

use crate::{
    evm_env::factory::HostEvmEnvFactory,
    test_harness::{
        contracts::{AVERAGE_BALANCE_OF_CALL, SIMPLE_TIME_TRAVEL},
        rpc_file_cache,
    },
    Call,
};

#[tokio::test]
async fn time_travel() -> anyhow::Result<()> {
    let chain_id = Chain::optimism_sepolia().id();

    let call = Call::new(SIMPLE_TIME_TRAVEL, AVERAGE_BALANCE_OF_CALL);

    let test_name = "simple_time_travel";
    let location: ExecutionLocation = (20064547, NamedChain::OptimismSepolia.into()).into();
    let provider_factory = CachedProviderFactory::new(rpc_file_cache(test_name), None);
    let provider = provider_factory.create(chain_id)?;
    let profiling_provider = Arc::new(ProfilingProvider::new(provider));
    let multi_provider = CachedMultiProvider::from_provider(chain_id, profiling_provider.clone());
    let envs = CachedEvmEnv::from_factory(HostEvmEnvFactory::new(multi_provider));
    let _ = TravelCallExecutor::new(&envs).call(&call, location);

    assert_eq!(profiling_provider.call_count(), 88);

    Ok(())
}
