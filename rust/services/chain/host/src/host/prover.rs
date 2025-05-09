use std::result;

use anyhow::bail;
use chain_common::ChainProofReceipt;
use chain_guest::Input;
use common::GuestElf;
use host_utils::{ProofMode, Prover as Risc0Prover};
use risc0_zkvm::{ExecutorEnv, InnerReceipt, ProveInfo};
use thiserror::Error;
use tracing::instrument;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[error("Prover: {0}")]
    Prover(String),
    #[error("ExecutorEnvBuilder: {0}")]
    ExecutorEnvBuilder(String),
}

impl From<host_utils::ProverError> for Error {
    fn from(err: host_utils::ProverError) -> Self {
        Self::Prover(err.to_string())
    }
}

type Result<T> = result::Result<T, Error>;

#[derive(Debug, Clone, Default)]
pub struct Prover {
    inner: Risc0Prover,
    elf: GuestElf,
}

impl Prover {
    pub fn try_new(proof_mode: ProofMode, elf: GuestElf) -> Result<Self> {
        Ok(Self {
            inner: Risc0Prover::try_new(proof_mode)?,
            elf,
        })
    }

    /// Wrapper around Risc0Prover which specifies the chain guest ELF and accepts the previous proof
    #[instrument(skip_all)]
    pub fn prove(
        &self,
        input: &Input,
        previous_proof: Option<ChainProofReceipt>,
    ) -> Result<ChainProofReceipt> {
        let executor_env = build_executor_env(input, previous_proof, self.inner.mode)
            .map_err(|err| Error::ExecutorEnvBuilder(err.to_string()))?;

        let ProveInfo { receipt, .. } = self.inner.prove(executor_env, &self.elf.elf)?;
        Ok(receipt.into())
    }
}

fn build_executor_env(
    input: &Input,
    assumption: Option<ChainProofReceipt>,
    proof_mode: ProofMode,
) -> anyhow::Result<ExecutorEnv<'static>> {
    let mut builder = ExecutorEnv::builder();
    if let Some(assumption) = assumption {
        validate_proof_mode_coherence(proof_mode, &assumption.inner)?;
        builder.add_assumption(assumption);
    }
    builder.write(&input)?.build()
}

fn validate_proof_mode_coherence(
    proof_mode: ProofMode,
    assumption_receipt: &InnerReceipt,
) -> anyhow::Result<()> {
    use InnerReceipt::*;
    match assumption_receipt {
        Fake(_) if proof_mode == ProofMode::Fake => Ok(()),
        Succinct(_) if proof_mode == ProofMode::Succinct => Ok(()),

        Fake(_) => bail!("Trying to include a fake proof within {} proof", proof_mode),
        Succinct(_) => bail!("Trying to include a succinct proof within {} proof", proof_mode),

        Composite(_) | Groth16(_) => bail!(
            "Trying to include a {} proof within {} proof. One can only compose fake or succinct proofs",
            match assumption_receipt {
                Composite(_) => "composite",
                Groth16(_) => "Groth16",
                _ => unreachable!(),
            },
            proof_mode
        ),

        _ => unreachable!("Unknown proof mode in assumption receipt: {:?}", assumption_receipt),
    }
}
