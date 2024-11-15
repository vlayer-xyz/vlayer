use std::{collections::HashMap, env};

use alloy_chains::{Chain, NamedChain};
use alloy_primitives::ChainId;
use alloy_sol_types::SolCall;
use call_guest_wrapper::GUEST_ELF;
use call_host::{
    host::{config::HostConfig, error::HostError, get_block_header, Host},
    Call,
};
use chain_client::RpcClient as RpcChainProofClient;
use dotenvy::dotenv;
use ethers_core::types::BlockNumber as BlockTag;
use lazy_static::lazy_static;
use mock_chain_server::{fake_proof_result, ChainProofServerMock};
use provider::{BlockNumber, CachedMultiProvider, CachedProviderFactory};
use serde_json::json;

pub mod contracts;

pub struct ExecutionLocation {
    pub chain_id: ChainId,
    pub block_tag: BlockTag,
}

impl<C, B> From<(C, B)> for ExecutionLocation
where
    C: Into<ChainId>,
    B: Into<BlockTag>,
{
    fn from((chain_id, block_tag): (C, B)) -> Self {
        ExecutionLocation {
            chain_id: chain_id.into(),
            block_tag: block_tag.into(),
        }
    }
}

// To activate recording, set UPDATE_SNAPSHOTS to true.
// Recording creates new testdata directory and writes return data from Alchemy into files in that directory.
const UPDATE_SNAPSHOTS: bool = false;
pub const LATEST_BLOCK: BlockTag = BlockTag::Latest;

fn get_alchemy_key() -> String {
    dotenv().ok();
    env::var("ALCHEMY_KEY").expect(
        "To use recording provider you need to set ALCHEMY_KEY in an .env file. See .env.example",
    )
}

lazy_static! {
    static ref alchemy_key: String = get_alchemy_key();
    static ref mainnet_url: String =
        format!("https://eth-mainnet.g.alchemy.com/v2/{}", *alchemy_key);
    static ref sepolia_url: String =
        format!("https://eth-sepolia.g.alchemy.com/v2/{}", *alchemy_key);
    static ref anvil_url: String = format!("http://localhost:8545");
    pub static ref sepolia_latest_block: ExecutionLocation =
        (Chain::sepolia().id(), LATEST_BLOCK).into();
}

fn rpc_file_cache(test_name: &str) -> HashMap<ChainId, String> {
    HashMap::from([
        (Chain::mainnet().id(), format!("testdata/mainnet_{test_name}_rpc_cache.json")),
        (Chain::sepolia().id(), format!("testdata/sepolia_{test_name}_rpc_cache.json")),
        (
            NamedChain::AnvilHardhat.into(),
            format!("testdata/anvil_{test_name}_rpc_cache.json"),
        ),
    ])
}

fn rpc_urls() -> HashMap<ChainId, String> {
    HashMap::from([
        (Chain::mainnet().id(), mainnet_url.clone()),
        (Chain::sepolia().id(), sepolia_url.clone()),
        (NamedChain::AnvilHardhat.into(), anvil_url.clone()),
    ])
}

pub async fn run<C>(
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
    let host_output = host.main(call).await?;
    let return_value = C::abi_decode_returns(&host_output.guest_output.evm_call_result, false)?;

    chain_proof_server.assert();

    Ok(return_value)
}

fn create_multi_provider(test_name: &str) -> CachedMultiProvider {
    let maybe_ethers_provider_factory =
        UPDATE_SNAPSHOTS.then(|| provider::EthersProviderFactory::new(rpc_urls()));
    let provider_factory =
        CachedProviderFactory::new(rpc_file_cache(test_name), maybe_ethers_provider_factory);
    CachedMultiProvider::new(provider_factory)
}

async fn create_chain_proof_server(
    multi_provider: &CachedMultiProvider,
    location: &ExecutionLocation,
) -> Result<ChainProofServerMock, HostError> {
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
) -> Result<Host, HostError> {
    let config = HostConfig {
        start_chain_id: location.chain_id,
        call_guest_elf: GUEST_ELF.clone(),
        ..Default::default()
    };
    let block_number =
        block_tag_to_block_number(&multi_provider, location.chain_id, location.block_tag)?;
    let chain_proof_client = RpcChainProofClient::new(chain_proof_server_url);
    Host::try_new_with_components(multi_provider, block_number, chain_proof_client, config)
}

fn block_tag_to_block_number(
    multi_provider: &CachedMultiProvider,
    chain_id: ChainId,
    block_tag: BlockTag,
) -> Result<BlockNumber, HostError> {
    match block_tag {
        BlockTag::Latest => {
            Ok(get_block_header(multi_provider, chain_id, BlockTag::Latest)?.number())
        }
        BlockTag::Number(block_no) => Ok(block_no.as_u64()),
        _ => panic!("Only Latest and specific block numbers are supported, got {:?}", block_tag),
    }
}

#[cfg(test)]
#[ctor::ctor]
fn before_all() {
    use std::{env::set_var, fs};
    set_var("RISC0_DEV_MODE", "1");

    if UPDATE_SNAPSHOTS {
        fs::remove_dir_all("testdata").ok();
        fs::create_dir("testdata").ok();
    }
}
