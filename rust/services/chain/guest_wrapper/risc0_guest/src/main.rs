#![no_main]

risc0_zkvm::guest::entry!(main);

use chain_guest::{main as guest_main, Input};
use risc0_zkvm::guest::env;

fn main() {
    let input: Input = env::read();
    let guest_output = guest_main(input);
    env::commit(&guest_output);
}
