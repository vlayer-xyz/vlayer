#![no_main]

risc0_zkvm::guest::entry!(main);

use chain_guest::{Guest, Input};
use risc0_zkvm::guest::env;

fn main() {
    let Input { root_hash } = env::read();

    let _ = Guest {};

    env::commit_slice(root_hash.as_slice());
}
