use bytes::Bytes;
use call_engine::Input;
use risc0_zkvm::ExecutorEnv;
use thiserror::Error;

pub trait CycleEstimator {
    fn estimate(&self, input: &Input, elf: Bytes) -> Result<u64, CycleEstimatorError>;
}

#[derive(Debug, Error)]
#[error("Cycle estimation failed: {0}")]
pub struct CycleEstimatorError(#[from] anyhow::Error);

#[derive(Debug, Default, Clone)]
pub struct Risc0CycleEstimator {}

impl Risc0CycleEstimator {
    pub const fn new() -> Self {
        Self {}
    }
}

impl CycleEstimator for Risc0CycleEstimator {
    fn estimate(&self, input: &Input, elf: Bytes) -> Result<u64, CycleEstimatorError> {
        let env = input
            .chain_proofs
            .values()
            .try_fold(ExecutorEnv::builder(), |mut builder, (_, proof)| {
                builder.add_assumption(proof.receipt.clone());
                Ok::<_, anyhow::Error>(builder)
            })?
            .write(input)?
            // Workaround for r0vm bug reproed in: https://github.com/vlayer-xyz/risc0-r0vm-fake-repro
            .segment_limit_po2(22)
            .build()?;

        let prover = risc0_zkvm::default_executor();

        let res = prover.execute(env, &elf)?;
        Ok(res.cycles())
    }
}

#[cfg(test)]
mod tests {
    use alloy_chains::Chain;
    use alloy_primitives::address;

    use super::*;
    use crate::test_harness::{
        ExecutionLocation, call,
        contracts::usdt::{BLOCK_NO, IERC20::balanceOfCall, USDT},
        preflight_raw_result,
    };

    mod estimate {
        use super::*;

        // `estimate` function is not deterministic, so we just check that result is greater than 0
        #[tokio::test(flavor = "multi_thread")]
        async fn result_greater_than_zero() -> anyhow::Result<()> {
            let location: ExecutionLocation = (Chain::mainnet().id(), BLOCK_NO).into();
            let binance_8 = address!("F977814e90dA44bFA03b6295A0616a897441aceC");
            let call = call(USDT, &balanceOfCall { account: binance_8 });
            let result = preflight_raw_result("usdt_erc20_balance_of", call, &location).await?;

            let gas_estimate =
                Risc0CycleEstimator::new().estimate(&result.input, result.guest_elf)?;

            assert!(gas_estimate > 0);

            Ok(())
        }

        #[tokio::test(flavor = "multi_thread")]
        async fn works_with_composed_proofs() -> anyhow::Result<()> {
            // todo
            Ok(())
        }
    }
}
