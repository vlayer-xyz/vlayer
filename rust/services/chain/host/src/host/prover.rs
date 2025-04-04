use std::result;

use chain_common::ChainProofReceipt;
use chain_guest::Input;
use common::GuestElf;
use host_utils::{ProofMode, Prover as Risc0Prover};
use risc0_zkvm::{ExecutorEnv, ProveInfo};
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
}

impl Prover {
    pub fn try_new(proof_mode: ProofMode, elf: GuestElf) -> Result<Self> {
        Ok(Self {
            inner: Risc0Prover::try_new(proof_mode, elf.elf)?,
        })
    }

    /// Wrapper around Risc0Prover which specifies the chain guest ELF and accepts the previous proof
    #[instrument(skip_all)]
    pub fn prove(
        &self,
        input: &Input,
        previous_proof: Option<ChainProofReceipt>,
    ) -> Result<ChainProofReceipt> {
        let executor_env = build_executor_env(input, previous_proof)
            .map_err(|err| Error::ExecutorEnvBuilder(err.to_string()))?;

        let ProveInfo { receipt, .. } = self.inner.prove(executor_env)?;
        Ok(receipt.into())
    }
}

fn build_executor_env(
    input: &Input,
    assumption: Option<ChainProofReceipt>,
) -> anyhow::Result<ExecutorEnv<'static>> {
    let mut builder = ExecutorEnv::builder();
    if let Some(assumption) = assumption {
        builder.add_assumption(assumption);
    }
    builder.write(&input)?.build()
}
