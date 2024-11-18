use std::result;

use bytes::Bytes;
use call_engine::Input;
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
    pub fn prove(&self, input: &Input) -> Result<Receipt> {
        let executor_env =
            build_executor_env(input).map_err(|err| Error::ExecutorEnvBuilder(err.to_string()))?;

        let ProveInfo { receipt, .. } = self
            .prover
            .prove(executor_env, &self.guest_elf)
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
    use call_guest_wrapper::GUEST_ELF;

    use super::*;

    #[test]
    fn invalid_input() {
        let res = Prover::new(ProofMode::Fake, &GUEST_ELF).prove(&Input::default());

        assert!(matches!(
            res.map(|_| ()).unwrap_err(),
            Error::Prover(ref msg) if msg == "failed to fill whole buffer"
        ));
    }
}
