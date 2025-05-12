use anyhow::bail;
use bytes::Bytes;
use call_engine::Input;
use host_utils::{ProofMode, Prover as Risc0Prover, proving};
use risc0_zkvm::{ExecutorEnv, InnerReceipt, ProveInfo};
use tracing::instrument;

#[derive(Debug, Clone, Default)]
pub struct Prover {
    prover: Risc0Prover,
    guest_elf: Bytes,
}

impl Prover {
    pub fn try_new(
        proof_mode: ProofMode,
        guest_elf: impl AsRef<Bytes>,
    ) -> Result<Self, crate::BuilderError> {
        Ok(Self {
            prover: Risc0Prover::try_new(proof_mode)?,
            guest_elf: guest_elf.as_ref().clone(), // Bytes is cheap to clone
        })
    }

    /// Wrapper around Risc0Prover which specifies the call guest ELF
    #[instrument(skip_all)]
    pub fn prove(&self, input: &Input) -> proving::Result<ProveInfo> {
        let executor_env = build_executor_env(input, self.prover.mode)?;
        Ok(self.prover.prove(executor_env, &self.guest_elf)?)
    }
}

fn build_executor_env(
    input: &Input,
    proof_mode: ProofMode,
) -> anyhow::Result<ExecutorEnv<'static>> {
    input
        .chain_proofs
        .values()
        .try_fold(ExecutorEnv::builder(), |mut builder, (_, proof)| {
            validate_proof_mode_coherence(proof_mode, &proof.receipt.inner)?;
            builder.add_assumption(proof.receipt.clone());
            Ok::<_, anyhow::Error>(builder)
        })?
        .write(&input)?
        // Workaround for r0vm bug reproed in: https://github.com/vlayer-xyz/risc0-r0vm-fake-repro
        .segment_limit_po2(22)
        .build()
}

fn validate_proof_mode_coherence(
    proof_mode: ProofMode,
    assumption_receipt: &InnerReceipt,
) -> anyhow::Result<()> {
    use InnerReceipt::*;
    match assumption_receipt {
        Fake(_) if proof_mode == ProofMode::Fake => Ok(()),
        Succinct(_) if proof_mode == ProofMode::Groth16 => Ok(()),
        Fake(_) => bail!("Trying to include a fake proof within {} proof", proof_mode),
        Succinct(_) => bail!("Trying to include a succinct proof within {} proof", proof_mode),
        Composite(_) | Groth16(_) => bail!(
            "Trying to include a {} proof within {} proof. One can only use fake or succinct proofs as assumptions",
            match assumption_receipt {
                Composite(_) => "composite",
                Groth16(_) => "Groth16",
                _ => unreachable!(), // already matched above
            },
            proof_mode
        ),
        _ => unreachable!("Unknown proof mode in assumption receipt: {:?}", assumption_receipt),
    }
}

#[cfg(test)]
mod tests {
    use guest_wrapper::CALL_GUEST_ELF;

    use super::*;

    #[test]
    fn invalid_input() {
        let res = Prover::try_new(ProofMode::Fake, &CALL_GUEST_ELF)
            .unwrap()
            .prove(&Input::default());

        assert_eq!(
            res.map(|_| ()).unwrap_err().to_string(),
            "Prover: Guest panicked: travel call verification failed: Teleport(Conversion(UnsupportedChainId(0)))"
        );
    }
}
