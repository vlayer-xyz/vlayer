use std::{collections::HashMap, env};

use alloy_chains::{Chain, NamedChain};
use alloy_primitives::ChainId;
use dotenvy::dotenv;
use ethers_core::types::BlockNumber as BlockTag;
use lazy_static::lazy_static;
use provider::{BlockNumber, CachedMultiProvider, CachedProviderFactory};

use crate::BuilderError;

// To activate recording, set UPDATE_SNAPSHOTS to true.
// Recording creates new test data directory and writes return data from Alchemy into files in that directory.
const UPDATE_SNAPSHOTS: bool = false;

fn get_alchemy_key() -> String {
    dotenv().ok();
    env::var("ALCHEMY_KEY").expect(
        "To use recording provider you need to set ALCHEMY_KEY in an .env file. See .env.example",
    )
}

fn get_quicknode_key() -> String {
    dotenv().ok();
    env::var("QUICKNODE_KEY").expect(
        "To use sequencer client you need to set QUICKNODE_KEY in an .env file. See .env.example",
    )
}

lazy_static! {
    static ref alchemy_key: String = get_alchemy_key();
    static ref mainnet_url: String =
        format!("https://eth-mainnet.g.alchemy.com/v2/{}", *alchemy_key);
    static ref sepolia_url: String =
        format!("https://eth-sepolia.g.alchemy.com/v2/{}", *alchemy_key);
    static ref op_mainnet_url: String =
        format!("https://opt-mainnet.g.alchemy.com/v2/{}", *alchemy_key);
    static ref op_sepolia_url: String =
        format!("https://opt-sepolia.g.alchemy.com/v2/{}", *alchemy_key);
    static ref base_mainnet_url: String =
        format!("https://base-mainnet.g.alchemy.com/v2/{}", *alchemy_key);
    static ref base_sepolia_url: String =
        format!("https://base-sepolia.g.alchemy.com/v2/{}", *alchemy_key);
    static ref world_sepolia_url: String =
        format!("https://world-sepolia.g.alchemy.com/v2/{}", *alchemy_key);
    static ref world_mainnet_url: String =
        format!("https://world-mainnet.g.alchemy.com/v2/{}", *alchemy_key);
    static ref unichain_sepolia_url: String =
        format!("https://unichain-sepolia.g.alchemy.com/v2/{}", *alchemy_key);
    static ref unichain_mainnet_url: String =
        format!("https://unichain-mainnet.g.alchemy.com/v2/{}", *alchemy_key);
    static ref anvil_url: String = format!("http://localhost:8545");
    static ref op_anvil_url: String = format!("http://localhost:8546");
    static ref quicknode_key: String = get_quicknode_key();
    pub static ref quicknode_op_sepolia_url: String = format!(
        "https://thrumming-burned-butterfly.optimism-sepolia.quiknode.pro/{}",
        *quicknode_key
    );
}

pub const OP_ANVIL: ChainId = 31_338;

pub fn rpc_cache_path(chain: &str, test_name: &str) -> String {
    format!("test_data/{test_name}/{chain}.json")
}

pub fn rpc_cache_paths(test_name: &str) -> HashMap<ChainId, String> {
    HashMap::from([
        (Chain::mainnet().id(), rpc_cache_path("mainnet", test_name)),
        (Chain::sepolia().id(), rpc_cache_path("sepolia", test_name)),
        (Chain::optimism_mainnet().id(), rpc_cache_path("op_mainnet", test_name)),
        (Chain::optimism_sepolia().id(), rpc_cache_path("op_sepolia", test_name)),
        (Chain::base_mainnet().id(), rpc_cache_path("base_mainnet", test_name)),
        (Chain::base_sepolia().id(), rpc_cache_path("base_sepolia", test_name)),
        (
            Chain::from_named(NamedChain::WorldSepolia).id(),
            rpc_cache_path("world_sepolia", test_name),
        ),
        (
            Chain::from_named(NamedChain::World).id(),
            rpc_cache_path("world_mainnet", test_name),
        ),
        (
            Chain::from_named(NamedChain::UnichainSepolia).id(),
            rpc_cache_path("unichain_sepolia", test_name),
        ),
        (
            Chain::from_named(NamedChain::Unichain).id(),
            rpc_cache_path("unichain_mainnet", test_name),
        ),
        (NamedChain::AnvilHardhat.into(), rpc_cache_path("anvil", test_name)),
        (OP_ANVIL, rpc_cache_path("op_anvil", test_name)),
    ])
}

pub fn rpc_urls() -> HashMap<ChainId, String> {
    HashMap::from([
        (Chain::mainnet().id(), mainnet_url.clone()),
        (Chain::sepolia().id(), sepolia_url.clone()),
        (Chain::optimism_mainnet().id(), op_mainnet_url.clone()),
        (Chain::optimism_sepolia().id(), op_sepolia_url.clone()),
        (Chain::base_mainnet().id(), base_mainnet_url.clone()),
        (Chain::base_sepolia().id(), base_sepolia_url.clone()),
        (Chain::from_named(NamedChain::WorldSepolia).id(), world_sepolia_url.clone()),
        (Chain::from_named(NamedChain::World).id(), world_mainnet_url.clone()),
        (
            Chain::from_named(NamedChain::UnichainSepolia).id(),
            unichain_sepolia_url.clone(),
        ),
        (Chain::from_named(NamedChain::Unichain).id(), unichain_mainnet_url.clone()),
        (NamedChain::AnvilHardhat.into(), anvil_url.clone()),
        (OP_ANVIL, op_anvil_url.clone()),
    ])
}

pub fn block_tag_to_block_number(
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

pub fn create_multi_provider(test_name: &str) -> CachedMultiProvider {
    let rpc_cache_paths = rpc_cache_paths(test_name);
    let maybe_ethers_provider_factory =
        UPDATE_SNAPSHOTS.then(|| provider::EthersProviderFactory::new(rpc_urls()));
    let provider_factory =
        CachedProviderFactory::new(rpc_cache_paths, maybe_ethers_provider_factory);
    CachedMultiProvider::from_factory(provider_factory)
}
