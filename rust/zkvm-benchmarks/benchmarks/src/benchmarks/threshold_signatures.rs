use crate::{benchmarks_fn, benchmarks_mod, Benchmark};

mod filecoin;
mod zcash;

benchmarks_fn! {
    benchmarks_mod!(zcash)
    benchmarks_mod!(filecoin)
}
