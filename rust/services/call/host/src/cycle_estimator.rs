use anyhow::Context;
use bytes::Bytes;
use call_engine::Input;
use risc0_zkvm::{ExecutorEnv, default_executor};
use thiserror::Error;

pub trait Estimator {
    fn estimate(&self, input: &Input, elf: Bytes) -> Result<u64, Error>;
}

#[derive(Debug, Error)]
#[error("Cycle estimation failed: {0}")]
pub struct Error(#[from] anyhow::Error);

#[derive(Debug, Default, Clone)]
pub struct Risc0Estimator;

impl Estimator for Risc0Estimator {
    fn estimate(&self, input: &Input, elf: Bytes) -> Result<u64, Error> {
        let env = build_executor_env(input)?;
        let executor = default_executor();

        let res = executor
            .execute(env, &elf)
            .context("failed to execute executor env")?;
        let total_cycles: u64 = res.segments.iter().map(|s| 1u64 << s.po2).sum();
        Ok(total_cycles)
    }
}

fn build_executor_env(input: &Input) -> Result<ExecutorEnv, anyhow::Error> {
    input
        .chain_proofs
        .values()
        .try_fold(ExecutorEnv::builder(), |mut builder, (_, proof)| {
            builder.add_assumption(proof.receipt.clone());
            Ok::<_, anyhow::Error>(builder)
        })
        .context("failed to add assumptions to executor env")?
        .write(input)?
        // Workaround for r0vm bug reproed in: https://github.com/vlayer-xyz/risc0-r0vm-fake-repro
        .segment_limit_po2(22)
        .build()
        .context("failed to build executor env")
}

#[cfg(test)]
mod tests {
    use alloy_chains::Chain;
    use alloy_primitives::address;

    use super::*;
    use crate::test_harness::{
        ExecutionLocation, call,
        contracts::{
            time_travel::{AVERAGE_BALANCE_OF_CALL, SIMPLE_TIME_TRAVEL},
            usdt::{IERC20::balanceOfCall, USDT},
        },
        preflight_raw_result,
    };

    // `estimate` function is not deterministic, so we just check that result is greater than 0
    mod estimate {
        use super::*;
        use crate::test_harness::contracts::{time_travel, usdt};

        // Below test is based on `erc20_balance_of` test from `preflight` module
        #[tokio::test(flavor = "multi_thread")]
        async fn result_greater_than_zero() -> anyhow::Result<()> {
            let location: ExecutionLocation = (Chain::mainnet().id(), usdt::BLOCK_NO).into();
            let binance_8 = address!("F977814e90dA44bFA03b6295A0616a897441aceC");
            let call = call(USDT, &balanceOfCall { account: binance_8 });
            let result = preflight_raw_result("usdt_erc20_balance_of", call, &location).await?;

            let cycle_estimate = Risc0Estimator.estimate(&result.input, result.guest_elf)?;

            assert!(cycle_estimate > 0);

            Ok(())
        }

        // Below test is based on `time_travel` test from `preflight`
        // module as time-travel example uses composed proofs.
        #[tokio::test(flavor = "multi_thread")]
        async fn works_with_composed_proofs() -> anyhow::Result<()> {
            let location: ExecutionLocation =
                (Chain::optimism_sepolia().id(), time_travel::BLOCK_NO).into();
            let call = call(SIMPLE_TIME_TRAVEL, &AVERAGE_BALANCE_OF_CALL);

            let result = preflight_raw_result("time_travel", call, &location).await?;

            let cycle_estimate = Risc0Estimator.estimate(&result.input, result.guest_elf)?;

            assert!(cycle_estimate > 0);

            Ok(())
        }
    }

    mod create_executor_env {
        use super::*;

        #[test]
        fn success() {
            let input = Input::default();
            let env = build_executor_env(&input);
            assert!(env.is_ok());
        }
    }
}
