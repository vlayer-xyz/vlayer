use lazy_static::lazy_static;

use crate::Benchmark;

mod hash;
mod mpt;
mod precompiles;

pub fn merge<Benchmarks: IntoIterator<Item = Benchmark>>(
    benchmarks: impl IntoIterator<Item = (&'static str, Benchmarks)>,
) -> Vec<Benchmark> {
    benchmarks
        .into_iter()
        .flat_map(|(module, benchmarks)| {
            benchmarks
                .into_iter()
                .map(|benchmark| benchmark.nest(module))
        })
        .collect()
}

lazy_static! {
    pub static ref BENCHMARKS: Vec<Benchmark> = {
        merge([
            ("hash", hash::BENCHMARKS.clone()),
            ("mpt", mpt::BENCHMARKS.clone()),
            ("precompiles", precompiles::BENCHMARKS.clone()),
        ])
    };
}
