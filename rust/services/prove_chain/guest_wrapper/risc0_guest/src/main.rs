#![no_main]

risc0_zkvm::guest::entry!(main);

use prove_chain_guest::{Guest, Input};
use risc0_zkvm::guest::env;

fn main() {
    let Input {} = env::read();

    let _ = Guest {};

    // env::commit_slice(&[]);
}
