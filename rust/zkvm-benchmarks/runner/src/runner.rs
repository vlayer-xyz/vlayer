use anyhow::Result;
use host_utils::{ProofMode, Prover};
use risc0_zkvm::ExecutorEnv;

use crate::guest::RISC0_BENCHMARK_GUEST_ELF;

#[derive(Default)]
pub struct Runner {}

impl Runner {
    pub fn run(&self, args: ()) -> Result<()> {
        let prover = Prover::new(ProofMode::Fake);
        let env = ExecutorEnv::builder().write(&args)?.build()?;
        prover.prove(env, RISC0_BENCHMARK_GUEST_ELF)?;
        Ok(())
    }
}
