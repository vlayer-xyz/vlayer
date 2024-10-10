use anyhow::Result;
use host_utils::{ProofMode, Prover};
use risc0_zkvm::ExecutorEnv;

use crate::guest::RISC0_BENCHMARK_GUEST_ELF;

#[derive(Default)]
pub struct Bencher {}

impl Bencher {
    pub fn run(&self, input: ()) -> Result<()> {
        let prover = Prover::new(ProofMode::Fake);
        let env = ExecutorEnv::builder().write(&input)?.build()?;

        let result = prover.prove(env, RISC0_BENCHMARK_GUEST_ELF)?;

        Ok(())
    }
}
