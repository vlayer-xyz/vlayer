use zkvm_benchmarks_runner::Runner;

fn main() {
    let runner: Runner = Default::default();
    runner.run(()).expect("❌ Failed to run benchmarks:");
    println!("✅ Successfully run all benchmarks ")
}
