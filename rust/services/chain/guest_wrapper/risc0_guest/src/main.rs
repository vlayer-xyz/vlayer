#![no_main]

risc0_zkvm::guest::entry!(main);

use chain_guest::{main as guest_main, Input, ChainGuestVerifier};
use risc0_zkvm::guest::env;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let input: Input = env::read();
    let verifier = ChainGuestVerifier::new_risc0();
    let guest_output = guest_main(input, &verifier).await;
    env::commit(&guest_output);
}
