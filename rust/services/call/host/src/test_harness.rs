use alloy_sol_types::SolCall;
use call_engine::HostOutput;
use chain_client::RpcClient as RpcChainProofClient;
use guest_wrapper::{CALL_GUEST_ELF, CHAIN_GUEST_ELF};
use mock_chain_server::ChainProofServerMock;
use provider::CachedMultiProvider;
pub use rpc::{
    block_tag_to_block_number, create_multi_provider, rpc_snapshot_file, rpc_snapshot_files,
};
pub use types::ExecutionLocation;

use crate::{BuilderError, Call, Config, Error, Host, PreflightResult};

pub mod contracts;
mod rpc;
mod types;

pub async fn preflight<C>(
    test_name: &str,
    call: Call,
    location: &ExecutionLocation,
) -> anyhow::Result<C::Return>
where
    C: SolCall,
{
    let multi_provider = create_multi_provider(test_name);
    let chain_proof_server = create_chain_proof_server(&multi_provider, location).await?;
    let host = create_host(multi_provider, location, chain_proof_server.url())?;
    let PreflightResult { host_output, .. } = host.preflight(call).await?;
    let return_value = C::abi_decode_returns(&host_output, false)?;

    chain_proof_server.assert();

    Ok(return_value)
}

pub async fn run(
    test_name: &str,
    call: Call,
    location: &ExecutionLocation,
) -> Result<HostOutput, Error> {
    let multi_provider = create_multi_provider(test_name);
    let chain_proof_server = create_chain_proof_server(&multi_provider, location).await?;
    let host = create_host(multi_provider, location, chain_proof_server.url())?;
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
) -> Result<Host, BuilderError> {
    let config = Config {
        call_guest_elf: CALL_GUEST_ELF.clone(),
        chain_guest_elf: CHAIN_GUEST_ELF.clone(),
        ..Default::default()
    };
    let block_number =
        block_tag_to_block_number(&multi_provider, location.chain_id, location.block_tag)?;
    let chain_proof_client = Box::new(RpcChainProofClient::new(chain_proof_server_url));
    let start_exec_location = (location.chain_id, block_number).into();
    Ok(Host::new(multi_provider, start_exec_location, Some(chain_proof_client), config))
}
