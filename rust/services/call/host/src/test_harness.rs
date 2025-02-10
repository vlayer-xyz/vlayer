use alloy_sol_types::SolCall;
use call_engine::HostOutput;
use chain_client::RpcClient as RpcChainProofClient;
use guest_wrapper::{CALL_GUEST_ELF, CHAIN_GUEST_ELF};
use mock_chain_server::ChainProofServerMock;
use optimism::client::factory::cached;
use provider::CachedMultiProvider;
use rpc::create_multi_provider;
pub use rpc::{block_tag_to_block_number, rpc_cache_path, rpc_cache_paths};
pub use types::ExecutionLocation;

use crate::{BuilderError, Call, Config, Error, Host, PreflightResult};

pub mod contracts;
pub mod rpc;
mod types;

pub async fn preflight<C>(
    test_name: &str,
    call: Call,
    location: &ExecutionLocation,
) -> anyhow::Result<C::Return>
where
    C: SolCall,
{
    let op_client_factory = cached::Factory::default();
    preflight_with_teleport::<C>(test_name, call, location, op_client_factory).await
}

pub async fn preflight_with_teleport<C>(
    test_name: &str,
    call: Call,
    location: &ExecutionLocation,
    op_client_factory: impl optimism::client::IFactory + 'static,
) -> anyhow::Result<C::Return>
where
    C: SolCall,
{
    let multi_provider = create_multi_provider(test_name);
    let chain_proof_server = create_chain_proof_server(&multi_provider, location).await?;
    let host = create_host(multi_provider, location, chain_proof_server.url(), op_client_factory)?;
    let PreflightResult { host_output, .. } = host.preflight(call).await?;
    let return_value = C::abi_decode_returns(&host_output, true)?;

    chain_proof_server.assert();

    Ok(return_value)
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
    let chain_proof_server = create_chain_proof_server(&multi_provider, location).await?;
    let host = create_host(multi_provider, location, chain_proof_server.url(), op_client_factory)?;
    let result = host.main(call).await?;

    chain_proof_server.assert();

    Ok(result)
}

async fn create_chain_proof_server(
    multi_provider: &CachedMultiProvider,
    location: &ExecutionLocation,
) -> Result<ChainProofServerMock, BuilderError> {
    let block_header = multi_provider.get_block_header(location.chain_id, location.block_tag)?;

    let mut chain_proof_server = ChainProofServerMock::start().await;
    chain_proof_server
        .mock_single_block(location.chain_id, block_header)
        .await;

    Ok(chain_proof_server)
}

fn create_host(
    multi_provider: CachedMultiProvider,
    location: &ExecutionLocation,
    chain_proof_server_url: impl AsRef<str>,
    op_client_factory: impl optimism::client::IFactory + 'static,
) -> Result<Host, BuilderError> {
    let config = Config {
        call_guest_elf: CALL_GUEST_ELF.clone(),
        chain_guest_ids: vec![CHAIN_GUEST_ELF.id].into_boxed_slice(),
        ..Default::default()
    };
    let block_number =
        block_tag_to_block_number(&multi_provider, location.chain_id, location.block_tag)?;
    let chain_proof_client = Box::new(RpcChainProofClient::new(chain_proof_server_url));
    let start_exec_location = (location.chain_id, block_number).into();
    Host::try_new(
        multi_provider,
        start_exec_location,
        Some(chain_proof_client),
        op_client_factory,
        config,
    )
}
