use std::sync::Arc;

use alloy_chains::{Chain, NamedChain};
use alloy_sol_types::SolCall;
use call_host::{self, Call};
use provider::{CachedMultiProvider, CachedProviderFactory, ProfilingProvider, ProviderFactory};

use crate::harness::{
    contracts::{AVERAGE_BALANCE_OF_CALL, SIMPLE_TIME_TRAVEL},
    create_chain_proof_server, create_host, rpc_file_cache, LATEST_BLOCK,
};

#[tokio::test]
async fn time_travel() -> anyhow::Result<()> {
    let chain_id = Chain::optimism_sepolia().id();

    let call = Call {
        to: SIMPLE_TIME_TRAVEL,
        data: AVERAGE_BALANCE_OF_CALL.abi_encode(),
    };

    let test_name = "simple_time_travel";
    let location = &(NamedChain::OptimismSepolia, LATEST_BLOCK).into();
    let provider_factory = CachedProviderFactory::new(rpc_file_cache(test_name), None);
    let provider = provider_factory.create(chain_id)?;
    let profiling_provider = Arc::new(ProfilingProvider::new(provider));
    let multi_provider = CachedMultiProvider::from_provider(chain_id, profiling_provider.clone());
    let chain_proof_server = create_chain_proof_server(&multi_provider, location).await?;
    let host = create_host(multi_provider, location, chain_proof_server.url())?;
    let _ = host.main(call).await;

    assert_eq!(profiling_provider.call_count(), 127);

    Ok(())
}
