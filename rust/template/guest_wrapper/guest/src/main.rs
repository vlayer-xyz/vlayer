#![no_main]

risc0_zkvm::guest::entry!(main);

fn main() {
    println!("Hello, world!")
}
