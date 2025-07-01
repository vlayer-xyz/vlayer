use call_engine::Input;
use common::GuestElf;
use dyn_clone::{DynClone, clone_trait_object};
use risc0_zkvm::ExecutorEnv;
use thiserror::Error;

pub trait GasEstimator: DynClone + Send + Sync {
    fn estimate(&self, input: &Input, elf: &GuestElf) -> Result<u64, GasEstimatorError>;
}

clone_trait_object!(GasEstimator);

#[derive(Debug, Error)]
pub enum GasEstimatorError {
    #[error("Gas estimation failed: {0}")]
    EstimateGas(#[from] anyhow::Error),
}

#[derive(Debug, Default, Clone)]
pub struct Risc0GasEstimator {}

impl Risc0GasEstimator {
    pub const fn new() -> Self {
        Self {}
    }
}

impl GasEstimator for Risc0GasEstimator {
    fn estimate(&self, input: &Input, elf: &GuestElf) -> Result<u64, GasEstimatorError> {
        let env = input
            .chain_proofs
            .values()
            .try_fold(ExecutorEnv::builder(), |mut builder, (_, proof)| {
                builder.add_assumption(proof.receipt.clone());
                Ok::<_, anyhow::Error>(builder)
            })?
            .write(input)?
            .segment_limit_po2(22)
            .build()?;

        let prover = risc0_zkvm::default_executor();

        let res = prover.execute(env, &elf.elf)?;
        Ok(res.cycles())
    }
}

#[derive(Debug, Clone)]
pub struct FakeGasEstimator {
    gas_limit: u64,
}

impl FakeGasEstimator {
    #[cfg(test)]
    pub const fn new() -> Self {
        Self { gas_limit: 0 }
    }
}

impl GasEstimator for FakeGasEstimator {
    fn estimate(&self, _input: &Input, _elf: &GuestElf) -> Result<u64, GasEstimatorError> {
        Ok(self.gas_limit)
    }
}
