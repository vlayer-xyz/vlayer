use chain_db::ChainProofReceipt;
use chain_guest::Input;
use chain_guest_wrapper::RISC0_CHAIN_GUEST_ELF;
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

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Default)]
pub struct Prover(Risc0Prover);

impl Prover {
    pub const fn new(proof_mode: ProofMode) -> Self {
        Self(Risc0Prover::new(proof_mode))
    }

    /// Wrapper around Risc0Prover which specifier the chain guest ELF and accepts the previous proof
    #[instrument(skip_all)]
    pub fn prove(
        &self,
        input: &Input,
        previous_proof: Option<ChainProofReceipt>,
    ) -> Result<ChainProofReceipt> {
        let executor_env = build_executor_env(input, previous_proof)
            .map_err(|err| Error::ExecutorEnvBuilder(err.to_string()))?;

        let ProveInfo { receipt, .. } = self
            .0
            .prove(executor_env, RISC0_CHAIN_GUEST_ELF)
            .map_err(|err| Error::Prover(err.to_string()))?;
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
