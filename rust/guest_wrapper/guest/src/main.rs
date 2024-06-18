#![no_main]

risc0_zkvm::guest::entry!(main);

use alloy_primitives::{address, Address};
use guest::Guest;
use risc0_zkvm::guest::env;
use vlayer_steel::{contract::CallTxData, guest_input::GuestInput};

pub mod guest;

const CONTRACT: Address = address!("5fbdb2315678afecb367f032d93f642f64180aa3");

fn main() {
    let GuestInput {
        evm_input,
        call_data,
    } = env::read();

    let call_data = CallTxData::<()>::new_from_bytes(CONTRACT, call_data);

    let returns = Guest::new(evm_input).run(call_data);
    env::commit(&returns);
}
