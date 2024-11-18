use std::{
    collections::HashMap,
    env,
    fs::{create_dir, remove_dir_all},
};

use alloy_chains::{Chain, NamedChain};
use alloy_primitives::ChainId;
use call_host::{get_block_header, Error};
use dotenvy::dotenv;
use ethers_core::types::BlockNumber as BlockTag;
use lazy_static::lazy_static;
use provider::{BlockNumber, CachedMultiProvider, CachedProviderFactory};

// To activate recording, set UPDATE_SNAPSHOTS to true.
// Recording creates new testdata directory and writes return data from Alchemy into files in that directory.
const UPDATE_SNAPSHOTS: bool = false;

#[cfg(test)]
#[ctor::ctor]
fn before_all() {
    if UPDATE_SNAPSHOTS {
        remove_dir_all("testdata").ok();
        create_dir("testdata").ok();
    }
}

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
    static ref op_sepolia_url: String =
        format!("https://opt-sepolia.g.alchemy.com/v2/{}", *alchemy_key);
    static ref anvil_url: String = format!("http://localhost:8545");
}

fn rpc_file_cache(test_name: &str) -> HashMap<ChainId, String> {
    HashMap::from([
        (Chain::mainnet().id(), format!("testdata/mainnet_{test_name}_rpc_cache.json")),
        (Chain::sepolia().id(), format!("testdata/sepolia_{test_name}_rpc_cache.json")),
        (
            Chain::optimism_sepolia().id(),
            format!("testdata/op_sepolia_{test_name}_rpc_cache.json"),
        ),
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
        (Chain::optimism_sepolia().id(), op_sepolia_url.clone()),
        (NamedChain::AnvilHardhat.into(), anvil_url.clone()),
    ])
}

pub fn block_tag_to_block_number(
    multi_provider: &CachedMultiProvider,
    chain_id: ChainId,
    block_tag: BlockTag,
) -> Result<BlockNumber, Error> {
    match block_tag {
        BlockTag::Latest => {
            Ok(get_block_header(multi_provider, chain_id, BlockTag::Latest)?.number())
        }
        BlockTag::Number(block_no) => Ok(block_no.as_u64()),
        _ => panic!("Only Latest and specific block numbers are supported, got {block_tag:?}"),
    }
}

pub fn create_multi_provider(test_name: &str) -> CachedMultiProvider {
    let maybe_ethers_provider_factory =
        UPDATE_SNAPSHOTS.then(|| provider::EthersProviderFactory::new(rpc_urls()));
    let provider_factory =
        CachedProviderFactory::new(rpc_file_cache(test_name), maybe_ethers_provider_factory);
    CachedMultiProvider::new(provider_factory)
}
