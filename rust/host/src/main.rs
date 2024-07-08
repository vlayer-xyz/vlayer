use std::env;

use alloy_primitives::{address, Address};
use host::{Host, HostConfig, HostError};
use tracing::info;
use vlayer_engine::{config::SEPOLIA_ID, evm::env::ExecutionLocation, io::Call};

pub mod db;
pub mod host;
pub mod host_tests;
pub mod into_input;
pub mod provider;

const CONTRACT: Address = address!("5fbdb2315678afecb367f032d93f642f64180aa3");
const CALLER: Address = address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
const LOCALHOST_RPC_URL: &str = "http://localhost:8545";

#[derive(Debug)]
struct Config {
    block_no: u64,
}

fn main() -> Result<(), HostError> {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let config = config();

    info!("Running proving on block number: {}", config.block_no);

    // This is the abi encoded call data (lhs = 1, rhs = 2) for the sum function in the Simple contract.
    let raw_call_data: Vec<u8> = vec![
        202, 208, 137, 155, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
    ];

    let call_tx_data = Call {
        caller: CALLER,
        to: CONTRACT,
        data: raw_call_data.clone(),
    };

    let execution_location = ExecutionLocation::new(config.block_no, SEPOLIA_ID);
    let config = HostConfig::new(LOCALHOST_RPC_URL, execution_location);
    let _return_data = Host::try_new(config)?.run(call_tx_data)?;

    Ok(())
}

fn config() -> Config {
    let block_no = env::var("BLOCK_NO")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(2);

    Config { block_no }
}
