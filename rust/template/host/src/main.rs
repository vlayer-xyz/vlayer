use alloy_primitives::{address, Address, U256};
use alloy_sol_types::sol;
use anyhow::Context;
use guest_wrapper::GUEST_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv};
use vlayer_steel::{config::ETH_SEPOLIA_CHAIN_SPEC, ethereum::EthEvmEnv, Contract};

sol! {
    interface Simple {
        function sum(uint256 lhs, uint256 rhs) public pure returns (uint256);
    }
}

const CALLER: Address = address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
const CONTRACT: Address = address!("5fbdb2315678afecb367f032d93f642f64180aa3");

fn main() -> anyhow::Result<()> {
    let call: Simple::sumCall = Simple::sumCall {
        lhs: U256::from(1),
        rhs: U256::from(2),
    };
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let mut env = EthEvmEnv::from_rpc("http://localhost:8545", None)?;
    env = env.with_chain_spec(&ETH_SEPOLIA_CHAIN_SPEC);
    let mut contract = Contract::preflight(CONTRACT, &mut env);
    let returns = contract.call_builder(&call).from(CALLER).call()?;

    let input = env.into_input()?;

    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();

    let prover = default_prover();
    prover
        .prove(env, GUEST_ELF)
        .context("failed to run prover")?;
    Ok(())
}
