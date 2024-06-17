use alloy_primitives::{address, Address};
use anyhow::Context;
use guest_wrapper::GUEST_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv};
use vlayer_steel::{
    config::ETH_SEPOLIA_CHAIN_SPEC,
    contract::{call::evm_call, CallTxData},
    ethereum::EthEvmEnv,
    guest_input::GuestInput,
};

const CONTRACT: Address = address!("5fbdb2315678afecb367f032d93f642f64180aa3");
const CALLER: Address = address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266");

struct Host {}

impl Host {
    pub fn build_and_run(
        &self,
        raw_call_data: Vec<u8>,
        call_data: CallTxData<()>,
    ) -> anyhow::Result<()> {
        let mut env = EthEvmEnv::from_rpc("http://localhost:8545", None)?;
        env = env.with_chain_spec(&ETH_SEPOLIA_CHAIN_SPEC)?;
        let _returns = evm_call(call_data, &mut env)?;

        let evm_input = env.into_input()?;
        let input = GuestInput {
            evm_input,
            call_data: raw_call_data,
        };
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
}

fn main() -> anyhow::Result<()> {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    // This is the abi encoded call data (lhs = 1, rhs = 2) for the sum function in the Simple contract.
    let raw_call_data: Vec<u8> = vec![
        202, 208, 137, 155, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
    ];

    let mut call_data = CallTxData::<()>::new_from_bytes(CONTRACT, raw_call_data.clone());
    call_data.caller = CALLER;

    let host = Host {};
    host.build_and_run(raw_call_data, call_data)?;

    Ok(())
}
