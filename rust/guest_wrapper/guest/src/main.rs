#![no_main]

risc0_zkvm::guest::entry!(main);

use guest::Guest;
use risc0_zkvm::guest::env;
use vlayer_steel::guest::Input;

pub mod guest;

fn main() {
    let Input { evm_input, call } = env::read();

    let output = Guest::new(evm_input).run(call);

    env::commit(&output);
}
