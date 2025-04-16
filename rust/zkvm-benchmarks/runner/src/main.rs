#![allow(clippy::expect_used, clippy::unwrap_used)]

use std::process::exit;

mod cycle;
mod guest;
mod row;
mod runner;
mod tolerance;

fn main() {
    let result = runner::run();

    if let Err(err) = result {
        eprintln!("❌ Failed to run benchmarks: {err}");
        exit(1);
    }
    println!("✅ Successfully run all benchmarks")
}
