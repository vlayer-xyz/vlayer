use alloy_primitives::Bytes as Input;
use bytes::Bytes;
use host_utils::{proving, ProofMode, Prover as Risc0Prover};
use risc0_zkvm::{ExecutorEnv, ProveInfo};
use tracing::instrument;

#[derive(Debug, Clone, Default)]
pub struct Prover {
    prover: Risc0Prover,
    guest_elf: Bytes,
}

impl Prover {
    pub fn new(proof_mode: ProofMode, guest_elf: impl AsRef<Bytes>) -> Self {
        Self {
            prover: Risc0Prover::try_new(proof_mode).unwrap(),
            guest_elf: guest_elf.as_ref().clone(), // Bytes is cheap to clone
        }
    }

    /// Wrapper around Risc0Prover which specifies the call guest ELF
    #[instrument(skip_all)]
    pub fn prove(&self, input: &Input) -> proving::Result<ProveInfo> {
        let executor_env = build_executor_env(input)?;
        Ok(self.prover.prove(executor_env, &self.guest_elf)?)
    }
}

fn build_executor_env(input: &Input) -> anyhow::Result<ExecutorEnv<'static>> {
    ExecutorEnv::builder().write(input)?.build()
}
