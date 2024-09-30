#![no_main]

risc0_zkvm::guest::entry!(main);

use chain_guest::{initialize, Input};
use risc0_zkvm::guest::env;

fn main() {
    let output = match env::read() {
        Input::Initialize { block, elf_id } => initialize(elf_id, &*block),
    };
    env::commit(&output);
}
