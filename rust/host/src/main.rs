use alloy_primitives::{address, Address};
use alloy_sol_types::SolCall;
use anyhow::Context;
use guest_wrapper::GUEST_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv};
use vlayer_common::Simple::sumCall;
use vlayer_steel::{
    config::ETH_SEPOLIA_CHAIN_SPEC,
    contract::call_builder::{evm_call, CallBuilder as SteelCallBuilder},
    ethereum::EthEvmEnv,
};

const CONTRACT: Address = address!("5fbdb2315678afecb367f032d93f642f64180aa3");
const CALLER: Address = address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266");

fn main() -> anyhow::Result<()> {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let mut env = EthEvmEnv::from_rpc("http://localhost:8545", None)?;
    env = env.with_chain_spec(&ETH_SEPOLIA_CHAIN_SPEC);

    let call_data: Vec<u8> = vec![
        202, 208, 137, 155, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
    ];
    let call = <sumCall as SolCall>::abi_decode(&call_data, true).unwrap();

    let call_builder = SteelCallBuilder::new(CONTRACT, &call).from(CALLER);
    let _returns = evm_call(call_builder, &mut env)?;

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
