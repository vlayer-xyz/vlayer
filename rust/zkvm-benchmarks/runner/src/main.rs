use std::process::exit;

mod cycle;
#[cfg(feature = "risc0")]
mod guest;
mod row;
mod runner;
#[cfg(feature = "risc0")]
mod tolerance;

fn main() {
    let result = runner::run();

    if let Err(err) = result {
        eprintln!("❌ Failed to run benchmarks: {err}");
        exit(1);
    }
    println!("✅ Successfully run all benchmarks")
}
