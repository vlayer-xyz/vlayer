use anyhow::Result;
use host_utils::{ProofMode, Prover};
use risc0_zkvm::ExecutorEnv;

use zkvm_bencher_types::GuestOutput;

use crate::guest::RISC0_BENCHMARK_GUEST_ELF;

#[derive(Default)]
pub struct Bencher {}

impl Bencher {
    pub fn run(&self, input: ()) -> Result<GuestOutput> {
        let prover = Prover::new(ProofMode::Fake);
        let env = ExecutorEnv::builder().write(&input)?.build()?;

        let result = prover.prove(env, RISC0_BENCHMARK_GUEST_ELF)?;
        let guest_output: GuestOutput = result.receipt.journal.bytes.as_slice().try_into().unwrap();
        Ok(guest_output)
    }
}
