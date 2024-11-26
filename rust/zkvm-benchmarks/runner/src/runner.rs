use host_utils::{ProofMode, Prover};
use risc0_zkvm::{serde, ExecutorEnv};
use tabled::Table;
use thiserror::Error;
use zkvm_benchmarks::BenchmarkResult;

use crate::{guest::RISC0_BENCHMARK_GUEST_ELF, row::Row, tolerance::apply_tolerance};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Regression: {0}")]
    Regression(String),
    #[error("Risc0 error: {0}")]
    Risc0(#[from] anyhow::Error),
    #[error("Serialization error: {0}")]
    Serde(#[from] serde::Error),
}

#[derive(Default)]
pub struct Runner {}

impl Runner {
    pub fn run(&self, args: ()) -> Result<(), Error> {
        let prover = Prover::new(ProofMode::Fake);
        let env = ExecutorEnv::builder().write(&args)?.build()?;
        let result = prover.prove(env, RISC0_BENCHMARK_GUEST_ELF)?;
        let results: Vec<BenchmarkResult> = result.receipt.journal.decode()?;

        for result in &results {
            detect_regression(result)?;
        }

        let rows: Vec<Row> = results.into_iter().map(Into::into).collect();
        println!("{}", Table::new(rows));

        Ok(())
    }
}

fn detect_regression(result: &BenchmarkResult) -> Result<(), Error> {
    let snapshot_with_tolerance = apply_tolerance(result.snapshot_cycles);
    if result.actual_cycles > snapshot_with_tolerance {
        return Err(Error::Regression(format!(
            "Regression in benchmark {}: {} cycles vs {} snapshot cycles",
            result.name, result.actual_cycles, result.snapshot_cycles
        )));
    }
    Ok(())
}
