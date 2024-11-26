use std::process::exit;

mod cycle;
mod guest;
mod row;
mod runner;
mod tolerance;

use runner::Runner;

fn main() {
    let runner: Runner = Default::default();
    let result = runner.run(());

    if let Err(err) = result {
        eprintln!("❌ Failed to run benchmarks: {err}");
        exit(1);
    }
    println!("✅ Successfully run all benchmarks")
}
