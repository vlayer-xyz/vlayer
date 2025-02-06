use std::{collections::HashMap, env};

use alloy_chains::{Chain, NamedChain};
use alloy_primitives::ChainId;
use dotenvy::dotenv;
use ethers_core::types::BlockNumber as BlockTag;
use lazy_static::lazy_static;
use provider::{BlockNumber, CachedMultiProvider};

use crate::BuilderError;

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
    static ref op_mainnet_url: String =
        format!("https://opt-mainnet.g.alchemy.com/v2/{}", *alchemy_key);
    static ref op_sepolia_url: String =
        format!("https://opt-sepolia.g.alchemy.com/v2/{}", *alchemy_key);
    static ref anvil_url: String = format!("http://localhost:8545");
    static ref op_anvil_url: String = format!("http://localhost:8546");
}

pub const OP_ANVIL: ChainId = 31_338;

pub fn rpc_snapshot_path(chain: &str, test_name: &str) -> String {
    format!("test_data/{test_name}/{chain}.json")
}

pub fn rpc_snapshot_paths(test_name: &str) -> HashMap<ChainId, String> {
    HashMap::from([
        (Chain::mainnet().id(), rpc_snapshot_path("mainnet", test_name)),
        (Chain::sepolia().id(), rpc_snapshot_path("sepolia", test_name)),
        (Chain::optimism_mainnet().id(), rpc_snapshot_path("op_mainnet", test_name)),
        (Chain::optimism_sepolia().id(), rpc_snapshot_path("op_sepolia", test_name)),
        (NamedChain::AnvilHardhat.into(), rpc_snapshot_path("anvil", test_name)),
        (OP_ANVIL, rpc_snapshot_path("op_anvil", test_name)),
    ])
}

pub fn rpc_urls() -> HashMap<ChainId, String> {
    HashMap::from([
        (Chain::mainnet().id(), mainnet_url.clone()),
        (Chain::sepolia().id(), sepolia_url.clone()),
        (Chain::optimism_mainnet().id(), op_mainnet_url.clone()),
        (Chain::optimism_sepolia().id(), op_sepolia_url.clone()),
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
