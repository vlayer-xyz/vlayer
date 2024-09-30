#![no_main]

risc0_zkvm::guest::entry!(main);

use chain_guest::{Guest, Input};
use risc0_zkvm::guest::env;

fn main() {
    let output = match env::read() {
        Input::Initialize { block } => Guest::initialize(&*block),
    };
    env::commit(&output);
}
