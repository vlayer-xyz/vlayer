use std::{sync::Arc, time::Instant};

use alloy_sol_types::SolValue;
use bytes::Bytes;
use call_engine::{Input, Seal};
use host_utils::{proving, ProofMode, ProofProvider, Prover as Risc0Prover, SP1Prover};
use risc0_zkvm::{ExecutorEnv, ProveInfo};
use seal::{EncodableProof, EncodableReceipt};
use sp1_sdk::SP1Stdin;
use tracing::instrument;

use super::{EncodedProofWithStats, ProvingError};

#[derive(Clone)]
pub struct Prover {
    prover: Arc<dyn CallProver>,
}

impl Prover {
    pub fn try_new(
        proof_mode: ProofMode,
        proof_provider: ProofProvider,
        guest_elf: impl AsRef<Bytes>,
    ) -> Result<Self, crate::BuilderError> {
        // Bytes is cheap to clone
        match proof_provider {
            ProofProvider::Risc0 => {
                let prover =
                    Arc::new(Risc0Prover::try_new(proof_mode, guest_elf.as_ref().clone())?);
                Ok(Self { prover })
            }
            ProofProvider::SP1 => {
                let prover = Arc::new(SP1Prover::try_new(proof_mode, guest_elf.as_ref().clone())?);
                Ok(Self { prover })
            }
        }
    }

    /// Wrapper around Risc0Prover which specifies the call guest ELF
    #[instrument(skip_all)]
    pub fn prove(&self, input: &Input) -> Result<EncodedProofWithStats, ProvingError> {
        self.prover.prove(input)
    }
}

fn build_executor_env(input: &Input) -> anyhow::Result<ExecutorEnv<'static>> {
    input
        .chain_proofs
        .values()
        .try_fold(ExecutorEnv::builder(), |mut builder, (_, proof)| {
            builder.add_assumption(proof.receipt.clone());
            Ok::<_, anyhow::Error>(builder)
        })?
        .write(&input)?
        // Workaround for r0vm bug reproed in: https://github.com/vlayer-xyz/risc0-r0vm-fake-repro
        .segment_limit_po2(21)
        .build()
}

pub trait CallProver: Sync + Send {
    fn prove(&self, input: &Input) -> Result<EncodedProofWithStats, ProvingError>;
}

impl CallProver for Risc0Prover {
    fn prove(&self, input: &Input) -> Result<EncodedProofWithStats, ProvingError> {
        let executor_env = build_executor_env(input).map_err(proving::Error::ExecutorEnvBuilder)?;
        let now = Instant::now();
        let ProveInfo { receipt, stats, .. } =
            self.prove(executor_env).map_err(proving::Error::Prover)?;
        let elapsed_time = now.elapsed();

        let seal: Seal = EncodableReceipt::from(receipt.clone()).try_into()?;
        let seal: Bytes = seal.abi_encode().into();
        let raw_guest_output: Bytes = receipt.journal.bytes.into();

        Ok(EncodedProofWithStats::new(
            seal,
            raw_guest_output,
            stats.total_cycles,
            elapsed_time,
        ))
    }
}

impl CallProver for SP1Prover {
    fn prove(&self, input: &Input) -> Result<EncodedProofWithStats, ProvingError> {
        let mut stdin = SP1Stdin::new();

        stdin.write(input);

        let now = Instant::now();
        let proof = self.prove(&stdin).map_err(proving::Error::Prover)?;
        let elapsed_time = now.elapsed();

        let seal: Seal = EncodableProof::from(&proof.proof).try_into()?;
        let seal: Bytes = seal.abi_encode().into();

        Ok(EncodedProofWithStats::new(
            seal,
            proof.proof.public_values.to_vec().into(),
            proof
                .report
                .map(|r| r.total_instruction_count())
                .unwrap_or_default(),
            elapsed_time,
        ))
    }
}

#[cfg(test)]
mod tests {
    use guest_wrapper::CALL_GUEST_ELF;

    use super::*;

    #[test]
    fn invalid_input() {
        let res = Prover::try_new(ProofMode::Fake, ProofProvider::Risc0, &CALL_GUEST_ELF)
            .unwrap()
            .prove(&Input::default());

        assert_eq!(
            res.map(|_| ()).unwrap_err().to_string(),
            "Prover: Guest panicked: travel call verification failed: Teleport(Conversion(UnsupportedChainId(0)))"
        );
    }
}
