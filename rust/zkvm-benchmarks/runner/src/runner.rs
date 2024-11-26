use derivative::Derivative;
use host_utils::{ProofMode, Prover};
use risc0_zkvm::{serde, ExecutorEnv};
use tabled::Table;
use thiserror::Error;
use zkvm_benchmarks::BenchmarkResult;

use crate::{guest::GUEST_ELF, row::Row, tolerance::apply_tolerance};

#[derive(Debug, Error, Derivative)]
#[derivative(PartialEq, Eq)]
pub enum Error {
    #[error("Regression: {0}")]
    Regression(BenchmarkResult),
    #[error("Risc0 error: {0}")]
    Risc0(
        #[from]
        #[derivative(PartialEq = "ignore")]
        anyhow::Error,
    ),
    #[error("Serialization error: {0}")]
    Serde(#[from] serde::Error),
}

pub fn run() -> Result<(), Error> {
    let results = run_guest()?;

    for result in &results {
        detect_regression(result)?;
    }

    let rows: Vec<Row> = results.into_iter().map(Into::into).collect();
    println!("{}", Table::new(rows));

    Ok(())
}

fn run_guest() -> Result<Vec<BenchmarkResult>, Error> {
    let prover = Prover::new(ProofMode::Fake);
    let env = ExecutorEnv::builder().build()?;
    let result = prover.prove(env, &GUEST_ELF.elf.clone())?;
    Ok(result.receipt.journal.decode()?)
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
