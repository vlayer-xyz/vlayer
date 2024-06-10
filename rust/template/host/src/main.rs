use risc0_zkvm::{default_prover, ExecutorEnv};

use alloy_sol_types::sol;

sol! {
    interface Simple {
        function sum(uint256, uint256) public pure returns (uint256);
    }

}

fn main() {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let input: u32 = 42;
    let _env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let _prover = default_prover();
}
