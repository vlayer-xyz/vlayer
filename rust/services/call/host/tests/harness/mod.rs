use std::env::set_var;

use alloy_chains::Chain;
use alloy_sol_types::SolCall;
use call_engine::HostOutput;
use call_guest_wrapper::GUEST_ELF as CALL_GUEST_ELF;
use call_host::{get_block_header, Call, Error, Host, Config, PreflightResult};
use chain_client::RpcClient as RpcChainProofClient;
use chain_guest_wrapper::GUEST_ELF as CHAIN_GUEST_ELF;
use ethers_core::types::BlockNumber as BlockTag;
use lazy_static::lazy_static;
use mock_chain_server::{fake_proof_result, ChainProofServerMock};
use provider::CachedMultiProvider;
use rpc::{block_tag_to_block_number, create_multi_provider};
use serde_json::json;
use types::ExecutionLocation;

pub mod contracts;
mod rpc;
mod types;

pub const LATEST_BLOCK: BlockTag = BlockTag::Latest;

lazy_static! {
    pub static ref sepolia_latest_block: ExecutionLocation =
        (Chain::sepolia().id(), LATEST_BLOCK).into();
}

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
    host.main(call).await
}

async fn create_chain_proof_server(
    multi_provider: &CachedMultiProvider,
    location: &ExecutionLocation,
) -> Result<ChainProofServerMock, Error> {
    let block_header = get_block_header(multi_provider, location.chain_id, location.block_tag)?;
    let block_number = block_header.number();
    let result = fake_proof_result(block_header);

    let chain_proof_server_mock = ChainProofServerMock::start(
        json!({
            "chain_id": location.chain_id,
            "block_numbers": [block_number]
        }),
        result,
    )
    .await;

    Ok(chain_proof_server_mock)
}

fn create_host(
    multi_provider: CachedMultiProvider,
    location: &ExecutionLocation,
    chain_proof_server_url: impl AsRef<str>,
) -> Result<Host, Error> {
    let config = Config {
        start_chain_id: location.chain_id,
        call_guest_elf: CALL_GUEST_ELF.clone(),
        chain_guest_elf: CHAIN_GUEST_ELF.clone(),
        verify_chain_proofs: true,
        ..Default::default()
    };
    let block_number =
        block_tag_to_block_number(&multi_provider, location.chain_id, location.block_tag)?;
    let chain_proof_client = RpcChainProofClient::new(chain_proof_server_url);
    Host::try_new_with_components(multi_provider, block_number, chain_proof_client, config)
}

#[cfg(test)]
#[ctor::ctor]
fn before_all() {
    set_var("RISC0_DEV_MODE", "1");
}
