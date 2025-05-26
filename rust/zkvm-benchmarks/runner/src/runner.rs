use derivative::Derivative;
use host_utils::{ProofMode, Prover, proving};
use risc0_zkvm::{ExecutorEnv, ProveInfo, serde};
use tabled::Table;
use thiserror::Error;
use zkvm_benchmarks::BenchmarkResult;

use crate::{guest::GUEST_ELF, row::Row, tolerance::apply_tolerance};

#[derive(Debug, Error, Derivative)]
#[derivative(PartialEq, Eq)]
pub enum Error {
    #[error("Regression: {0}")]
    Regression(BenchmarkResult),
    #[error("Proving error: {0}")]
    Proving(#[from] proving::Error),
    #[error("Serialization error: {0}")]
    Serde(#[from] serde::Error),
}

pub fn run() -> Result<(), Error> {
    let results = run_guest()?;

    for result in &results {
        detect_regression(result)?;
    }

    println!("{}", Table::new(results.into_iter().map(Row::from)));

    Ok(())
}

fn run_guest() -> Result<Vec<BenchmarkResult>, Error> {
    let result = prove()?;
    Ok(result.receipt.journal.decode()?)
}

fn prove() -> proving::Result<ProveInfo> {
    let prover = Prover::try_new(ProofMode::Fake)?;
    let env = ExecutorEnv::builder().build()?;
    Ok(prover.prove(env, &GUEST_ELF.elf.clone())?)
}

fn detect_regression(result: &BenchmarkResult) -> Result<(), Error> {
    let snapshot_with_tolerance = apply_tolerance(result.snapshot_cycles);
    if result.actual_cycles > snapshot_with_tolerance {
        return Err(Error::Regression(result.clone()));
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    mod detect_regression {
        use super::*;

        #[test]
        fn slightly_more() {
            let result = BenchmarkResult::new("".to_string(), 110, 100);

            assert!(detect_regression(&result).is_ok());
        }

        #[test]
        fn regression() {
            let result = BenchmarkResult::new("".to_string(), 111, 100);

            assert_eq!(detect_regression(&result).unwrap_err(), Error::Regression(result));
        }
    }
}
