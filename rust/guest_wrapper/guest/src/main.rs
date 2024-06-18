#![no_main]

risc0_zkvm::guest::entry!(main);

use guest::Guest;
use risc0_zkvm::guest::env;
use vlayer_steel::{
    contract::CallTxData,
    guest_input::{Call, GuestInput},
};

pub mod guest;

fn main() {
    let GuestInput {
        evm_input,
        call: Call { caller, to, data },
    } = env::read();

    let call_data = CallTxData::new_from_bytes(caller, to, data);

    let returns = Guest::new(evm_input).run(call_data);
    env::commit(&returns);
}
