use alloy_primitives::{Address, ChainId};
use alloy_sol_types::SolCall;
use call_engine::HostOutput;
use call_rpc::{rpc_cache_paths, rpc_urls};
use guest_wrapper::{CALL_GUEST_ELF, CHAIN_GUEST_ELF};
use optimism::client::factory::cached::{self, Factory};
use provider::{BlockNumber, BlockTag, CachedMultiProvider, CachedProviderFactory};
// pub use rpc::{rpc_cache_path, rpc_cache_paths};
pub use types::ExecutionLocation;

use crate::{BuilderError, Call, Config, Error, Host, PreflightResult};

pub mod contracts;
mod types;

// To activate recording, set UPDATE_SNAPSHOTS to true.
// Recording creates new test data directory and writes return data from Alchemy into files in that directory.
const UPDATE_SNAPSHOTS: bool = false;

const GAS_LIMIT: u64 = 1_000_000;

pub async fn preflight<C>(
    test_name: &str,
    call: Call,
    location: &ExecutionLocation,
) -> anyhow::Result<C::Return>
where
    C: SolCall,
{
    preflight_with_factory::<C>(test_name, call, location, Factory::default()).await
}

pub async fn preflight_raw_result(
    test_name: &str,
    call: Call,
    location: &ExecutionLocation,
) -> anyhow::Result<PreflightResult, Error> {
    preflight_inner(test_name, call, location, Factory::default()).await
}

pub async fn preflight_with_factory<C>(
    test_name: &str,
    call: Call,
    location: &ExecutionLocation,
    op_client_factory: impl optimism::client::IFactory + 'static,
) -> anyhow::Result<C::Return>
where
    C: SolCall,
{
    let preflight_result = preflight_inner(test_name, call, location, op_client_factory).await?;
    let decoded_host_output = C::abi_decode_returns(&preflight_result.host_output, true)?;

    Ok(decoded_host_output)
}

async fn preflight_inner(
    test_name: &str,
    call: Call,
    location: &ExecutionLocation,
    op_client_factory: impl optimism::client::IFactory + 'static,
) -> Result<PreflightResult, Error> {
    let multi_provider = create_multi_provider(test_name);
    let host = create_host(multi_provider, location, op_client_factory)?;
    Ok(host.preflight(call).await?)
}

pub async fn run(
    test_name: &str,
    call: Call,
    location: &ExecutionLocation,
) -> Result<HostOutput, Error> {
    let op_client_factory = cached::Factory::default();
    run_with_teleport(test_name, call, location, op_client_factory).await
}

pub async fn run_with_teleport(
    test_name: &str,
    call: Call,
    location: &ExecutionLocation,
    op_client_factory: impl optimism::client::IFactory + 'static,
) -> Result<HostOutput, Error> {
    let multi_provider = create_multi_provider(test_name);
    let host = create_host(multi_provider, location, op_client_factory)?;
    let result = host.main(call).await?;

    Ok(result)
}

fn create_host(
    multi_provider: CachedMultiProvider,
    location: &ExecutionLocation,
    op_client_factory: impl optimism::client::IFactory + 'static,
) -> Result<Host, BuilderError> {
    let config = Config {
        call_guest_elf: CALL_GUEST_ELF.clone(),
        chain_guest_ids: vec![CHAIN_GUEST_ELF.id].into_boxed_slice(),
        ..Default::default()
    };
    let block_number =
        block_tag_to_block_number(&multi_provider, location.chain_id, location.block_tag)?;
    let chain_proof_client =
        Box::new(chain_client::FakeClient::new(multi_provider.clone(), CHAIN_GUEST_ELF.id));
    let start_exec_location = (location.chain_id, block_number).into();
    Host::try_new(
        multi_provider,
        start_exec_location,
        Some(chain_proof_client),
        op_client_factory,
        config,
    )
}

fn block_tag_to_block_number(
    multi_provider: &CachedMultiProvider,
    chain_id: ChainId,
    block_tag: BlockTag,
) -> Result<BlockNumber, BuilderError> {
    match block_tag {
        BlockTag::Latest => Ok(multi_provider
            .get_block_header(chain_id, BlockTag::Latest)?
            .number()),
        BlockTag::Number(block_no) => Ok(block_no.as_u64()),
        _ => panic!("Only Latest and specific block numbers are supported, got {block_tag:?}"),
    }
}

fn create_multi_provider(test_name: &str) -> CachedMultiProvider {
    let rpc_cache_paths = rpc_cache_paths(test_name);
    let maybe_ethers_provider_factory =
        UPDATE_SNAPSHOTS.then(|| provider::EthersProviderFactory::new(rpc_urls()));
    let provider_factory =
        CachedProviderFactory::new(rpc_cache_paths, maybe_ethers_provider_factory);
    CachedMultiProvider::from_factory(provider_factory)
}

pub fn call(to: Address, data: &impl SolCall) -> Call {
    Call::new(to, data, GAS_LIMIT)
}
