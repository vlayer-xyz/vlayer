use crate::{benchmarks_fn, benchmarks_mod, Benchmark};

mod zcash;

benchmarks_fn! {
    benchmarks_mod!(zcash)
}
