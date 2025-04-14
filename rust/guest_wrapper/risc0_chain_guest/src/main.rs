#![no_main]

risc0_zkvm::guest::entry!(main);

use chain_guest::{Input, main as guest_main};
use risc0_zkvm::guest::env;

include!(concat!(env!("OUT_DIR"), "/guest_id.rs"));

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let input: Input = env::read();
    let old_elf_ids = CHAIN_GUEST_IDS.into_iter().map(Into::into);
    let guest_output = guest_main(input, old_elf_ids).await;
    env::commit(&guest_output);
}
