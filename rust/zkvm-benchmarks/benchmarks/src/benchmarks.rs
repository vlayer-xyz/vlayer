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

pub fn benchmarks() -> Vec<Benchmark> {
    merge([
        ("hash", hash::benchmarks()),
        ("mpt", mpt::benchmarks()),
        ("precompiles", precompiles::benchmarks()),
    ])
}
