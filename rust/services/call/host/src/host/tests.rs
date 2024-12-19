use alloy_sol_types::SolCall;
use call_engine::Call;

mod number_of_rpc_calls;
mod preflight;
mod with_guest;

use alloy_primitives::Address;

const GAS_LIMIT: u64 = 1_000_000;

pub fn call(to: Address, data: &impl SolCall) -> Call {
    Call::new(to, data, GAS_LIMIT)
}
