use alloy_sol_types::sol;
use anyhow::Context;
use guest_wrapper::GUEST_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv};

sol! {
    interface Simple {
        function sum(uint256, uint256) public pure returns (uint256);
    }

}

fn main() -> anyhow::Result<()> {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let env = ExecutorEnv::builder().build().unwrap();

    let prover = default_prover();
    prover
        .prove(env, GUEST_ELF)
        .context("failed to run prover")?;
    Ok(())
}
