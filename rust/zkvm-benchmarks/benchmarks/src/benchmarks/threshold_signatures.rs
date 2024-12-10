use crate::{benchmarks_fn, benchmarks_mod, Benchmark};

mod filecoin;
mod rsa;
mod zcash;

benchmarks_fn! {
    benchmarks_mod!(filecoin),
    benchmarks_mod!(rsa),
    benchmarks_mod!(zcash)
}
