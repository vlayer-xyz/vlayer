use derivative::Derivative;
use host_utils::proving;
use tabled::Table;
use thiserror::Error;
use zkvm_benchmarks::BenchmarkResult;

use crate::row::Row;

#[cfg(feature = "risc0")]
use crate::guest::GUEST_ELF;

#[cfg(feature = "sp1")]
const SP1_GUEST_ELF: &[u8] = sp1_sdk::include_elf!("sp1_benchmark_guest");

#[derive(Debug, Error, Derivative)]
#[derivative(PartialEq, Eq)]
pub enum Error {
    #[allow(dead_code)]
    #[error("Regression: {0}")]
    Regression(BenchmarkResult),
    #[error("Proving error: {0}")]
    Proving(#[from] proving::Error),
    #[cfg(feature = "risc0")]
    #[error("Serialization error: {0}")]
    Serde(#[from] risc0_zkvm::serde::Error),
}

pub fn run() -> Result<(), Error> {
    #[cfg(feature = "sp1")]
    sp1_sdk::utils::setup_logger();

    let results = run_guest()?;

    #[cfg(feature = "risc0")]
    for result in &results {
        detect_regression(result)?;
    }

    println!("{}", Table::new(results.into_iter().map(Row::from)));

    Ok(())
}

fn run_guest() -> Result<Vec<BenchmarkResult>, Error> {
    let result = prove()?;
    Ok(result)
}

fn prove() -> Result<Vec<BenchmarkResult>, Error> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "risc0")] {
            use host_utils::{proving, ProofMode, Prover};

            let prover = Prover::try_new(ProofMode::Fake)
                .map_err(|err| Error::Proving(err.into()))?;
            let env = risc0_zkvm::ExecutorEnv::builder()
                .build()
                .map_err(|err| Error::Proving(err.into()))?;
            let result = prover
                .prove(env, &GUEST_ELF.elf.clone())
                .map_err(|err| Error::Proving(err.into()))?;

            Ok(result
                .receipt
                .journal
                .decode()
                .map_err(|err| Error::Serde(err))?)
        } else if #[cfg(feature = "sp1")] {
            use host_utils::{ProofMode, SP1Prover};

            let prover = SP1Prover::try_new(ProofMode::Fake, SP1_GUEST_ELF.to_vec())
                .map_err(|err| Error::Proving(err.into()))?;
            let stdin = sp1_sdk::SP1Stdin::new();
            let mut result = prover
                .prove(stdin)
                .map_err(|err| Error::Proving(err.into()))?;
            let mut benchmarks = result.public_values.read::<Vec<BenchmarkResult>>();

            for b in &mut benchmarks {
                b.actual_cycles = result.report.cycle_tracker[&b.name]
            }

            Ok(benchmarks)
        } else {
            panic!("Either 'risc0' or 'sp1' feature must be enabled")
        }
    }
}

#[cfg(feature = "risc0")]
fn detect_regression(result: &BenchmarkResult) -> Result<(), Error> {
    let snapshot_with_tolerance = crate::tolerance::apply_tolerance(result.snapshot_cycles);
    if result.actual_cycles > snapshot_with_tolerance {
        return Err(Error::Regression(result.clone()));
    }
    Ok(())
}
#[cfg(feature = "risc0")]
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
