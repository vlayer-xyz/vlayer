use std::process::exit;

use zkvm_benchmarks_runner::Runner;

fn main() {
    let runner: Runner = Default::default();
    let result = runner.run(());

    if let Err(err) = result {
        eprintln!("❌ Failed to run benchmarks: {}", err);
        exit(1);
    }
    println!("✅ Successfully run all benchmarks")
}
