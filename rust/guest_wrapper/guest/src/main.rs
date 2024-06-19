#![no_main]

risc0_zkvm::guest::entry!(main);

use guest::Guest;
use risc0_zkvm::guest::env;
use vlayer_steel::guest_input::GuestInput;

pub mod guest;

fn main() {
    let GuestInput { evm_input, call } = env::read();

    let returns = Guest::new(evm_input).run(call);
    env::commit(&returns);
}
