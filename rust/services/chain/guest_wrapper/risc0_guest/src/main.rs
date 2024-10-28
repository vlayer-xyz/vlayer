#![no_main]

risc0_zkvm::guest::entry!(main);

use chain_guest::{main as guest_main, Input};
use risc0_zkvm::guest::env;

fn main() {
    let input: Input = env::read();
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("failed to create tokio runtime");
    let guest_output = runtime.block_on(guest_main(input));
    env::commit(&guest_output);
}
