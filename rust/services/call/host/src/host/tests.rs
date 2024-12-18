use alloy_sol_types::SolCall;
use call_engine::Call;

mod number_of_rpc_calls;
mod preflight;
mod with_guest;

use alloy_primitives::Address;

/// The lowest possible power of 10 that enables all the tests to pass.
const GAS_LIMIT: u64 = 100_000_000_000_000;

pub fn call(to: Address, data: &impl SolCall) -> Call {
    Call::new(to, data, 100_000_000_000_000)
}
