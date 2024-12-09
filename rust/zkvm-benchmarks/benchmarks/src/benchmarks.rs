use crate::Benchmark;

// mod block_trie;
// mod hash;
// mod mpt;
// mod precompiles;
mod threshold_signatures;

#[macro_export]
macro_rules! with_fixture {
    ($fixture:expr, $callback:expr) => {{
        let fixture = $fixture;
        move || $callback(fixture)
    }};
}

#[macro_export]
macro_rules! benchmarks_mod {
    ($mod:ident) => {{
        (stringify!($mod), $mod::benchmarks())
    }};
}

#[macro_export]
macro_rules! benchmarks_fn {
    ($($benchmark_mod:expr),*) => {
        pub(crate) fn benchmarks() -> Vec<Benchmark> {
            use crate::benchmarks::merge;
            merge(vec![$($benchmark_mod),*])
        }
    };
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

pub fn benchmarks_() -> Vec<Benchmark> {
    merge([
        // benchmarks_mod!(hash),
        // benchmarks_mod!(mpt),
        // benchmarks_mod!(block_trie),
        // benchmarks_mod!(precompiles),
        benchmarks_mod!(threshold_signatures),
    ])
}
