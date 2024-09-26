use std::collections::HashMap;

use alloy_primitives::ChainId;
use anyhow::Result;
use clap::Parser;
use provider::{BlockTag, BlockingProvider, EthersProviderFactory, ProviderFactory};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    chain_id: ChainId,
    #[arg(long)]
    rpc_url: String,
}

fn main() -> Result<()> {
    let Args { chain_id, rpc_url } = Args::parse();

    let rpc_urls = HashMap::from([(chain_id, rpc_url)]);
    let provider_factory = EthersProviderFactory::new(rpc_urls);
    let provider = provider_factory.create(chain_id)?;
    let latest_block = provider
        .get_block_header(BlockTag::Latest)?
        .expect("block not found");

    println!("{:?}", latest_block.number());

    Ok(())
}
