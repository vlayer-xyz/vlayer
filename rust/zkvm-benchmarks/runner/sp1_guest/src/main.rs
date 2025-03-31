#![no_main]
sp1_zkvm::entrypoint!(main);

use zkvm_benchmarks::Runner;

fn main() {
    let runner: Runner = Default::default();

    sp1_zkvm::io::commit(&runner.run());
}
