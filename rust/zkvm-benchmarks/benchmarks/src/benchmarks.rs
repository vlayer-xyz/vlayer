use lazy_static::lazy_static;

use crate::Benchmark;

pub(crate) mod keccak;
pub(crate) mod mpt;
pub(crate) mod precompiles;
pub(crate) mod sha2;

pub fn merge<Benchmarks: IntoIterator<Item = Benchmark>>(
    benchmarks: impl IntoIterator<Item = (&'static str, Benchmarks)>,
) -> Vec<Benchmark> {
    benchmarks
        .into_iter()
        .flat_map(|(module, benchmarks)| {
            benchmarks.into_iter().map(move |benchmark| {
                Benchmark::new(
                    format!("{}::{}", module, benchmark.name),
                    benchmark.workload,
                    benchmark.total_cycles_limit,
                )
            })
        })
        .collect()
}

lazy_static! {
    pub static ref BENCHMARKS: Vec<Benchmark> = {
        merge([
            ("sha2", sha2::BENCHMARKS.clone()),
            ("keccak", keccak::BENCHMARKS.clone()),
            ("mpt", mpt::BENCHMARKS.clone()),
            ("precompiles", precompiles::BENCHMARKS.clone()),
        ])
    };
}
