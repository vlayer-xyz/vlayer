use bytes::Bytes;
use call_engine::Input;
use chain_common::ChainProofReceipt;
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
            prover: Risc0Prover::new(proof_mode),
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
    input
        .chain_proofs
        .values()
        .try_fold(ExecutorEnv::builder(), |mut builder, (_, proof)| {
            let receipt: ChainProofReceipt = proof.try_into()?;
            builder.add_assumption(receipt);
            Ok::<_, anyhow::Error>(builder)
        })?
        .write(&input)?
        .build()
}

#[cfg(test)]
mod tests {
    use call_guest_wrapper::GUEST_ELF;

    use super::*;

    #[test]
    fn invalid_input() {
        let res = Prover::new(ProofMode::Fake, &GUEST_ELF).prove(&Input::default());

        assert_eq!(
            res.map(|_| ()).unwrap_err().to_string(),
            "Prover: Guest panicked: travel call execution failed: EvmEnv(\"NullEvmEnvFactory cannot create EvmEnv\")"
        );
    }
}
