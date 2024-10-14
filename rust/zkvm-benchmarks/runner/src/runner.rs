use anyhow::{Error, Result};
use host_utils::{ProofMode, Prover};
use risc0_zkvm::ExecutorEnv;

use crate::guest::RISC0_BENCHMARK_GUEST_ELF;

#[derive(Default)]
pub struct Runner {}

impl Runner {
    pub fn run(&self, args: ()) -> Result<()> {
        let prover = Prover::new(ProofMode::Fake);
        let env = ExecutorEnv::builder().write(&args)?.build()?;
        let result = prover.prove(env, RISC0_BENCHMARK_GUEST_ELF)?;
        let status: u64 = result.receipt.journal.decode()?;

        if status == 0 {
            Ok(())
        } else {
            Err(Error::msg(format!("Execution failed for {} benchmarks", status)))
        }
    }
}
