use crate::Benchmark;

mod accelerators;
mod block_trie;
mod mpt;
mod precompiles;

#[macro_export]
macro_rules! with_fixture {
    ($fixture:expr, $callback:expr) => {{
        let fixture = $fixture;
        move || $callback(fixture)
    }};
}

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
        ("accelerators", accelerators::benchmarks()),
        ("mpt", mpt::benchmarks()),
        ("block_trie", block_trie::benchmarks()),
        ("precompiles", precompiles::benchmarks()),
    ])
}
