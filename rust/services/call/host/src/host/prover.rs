use std::result;

use call_engine::Input;
use call_guest_wrapper::RISC0_CALL_GUEST_ELF;
use chain_common::ChainProofReceipt;
use host_utils::{ProofMode, Prover as Risc0Prover};
use risc0_zkvm::{ExecutorEnv, ProveInfo, Receipt};
use thiserror::Error;
use tracing::instrument;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[error("Prover: {0}")]
    Prover(String),
    #[error("ExecutorEnvBuilder: {0}")]
    ExecutorEnvBuilder(String),
}

type Result<T> = result::Result<T, Error>;

#[derive(Debug, Clone, Default)]
pub struct Prover(Risc0Prover);

impl Prover {
    pub const fn new(proof_mode: ProofMode) -> Self {
        Self(Risc0Prover::new(proof_mode))
    }

    /// Wrapper around Risc0Prover which specifies the call guest ELF
    #[instrument(skip_all)]
    pub fn prove(&self, input: &Input) -> Result<Receipt> {
        let executor_env =
            build_executor_env(input).map_err(|err| Error::ExecutorEnvBuilder(err.to_string()))?;

        let ProveInfo { receipt, .. } = self
            .0
            .prove(executor_env, RISC0_CALL_GUEST_ELF)
            .map_err(|err| Error::Prover(err.to_string()))?;
        Ok(receipt)
    }
}

fn build_executor_env(input: &Input) -> anyhow::Result<ExecutorEnv<'static>> {
    input
        .chain_proofs
        .iter()
        .flat_map(|chain_proofs| chain_proofs.values())
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
    use super::*;

    #[test]
    fn invalid_input() {
        let res = Prover::default().prove(&Input::default());

        assert!(matches!(
            res.map(|_| ()).unwrap_err(),
            Error::Prover(ref msg) if msg == "Guest panicked: travel call execution failed: EvmEnv(\"NullEvmEnvFactory cannot create EvmEnv\")"
        ));
    }
}
