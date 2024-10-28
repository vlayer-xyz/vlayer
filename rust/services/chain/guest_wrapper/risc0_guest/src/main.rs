#![no_main]

risc0_zkvm::guest::entry!(main);

use chain_guest::{main as guest_main, Input};
use risc0_zkvm::guest::env;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let input: Input = env::read();
    let guest_output = guest_main(input).await;
    env::commit(&guest_output);
}
