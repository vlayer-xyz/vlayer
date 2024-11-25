use lazy_static::lazy_static;

use crate::{benchmarks::merge, Benchmark};
mod email;

lazy_static! {
    pub static ref BENCHMARKS: Vec<Benchmark> = merge([("email", email::BENCHMARKS.clone())]);
}
