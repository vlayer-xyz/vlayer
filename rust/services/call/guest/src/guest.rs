use call_engine::{
    evm::env::cached::CachedEvmEnv, travel_call::Executor as TravelCallExecutor, CallAssumptions,
    GuestOutput, Input,
};
use env::create_envs_from_input;

mod env;

pub async fn main(
    Input {
        multi_evm_input,
        start_execution_location,
        call,
    }: Input,
) -> GuestOutput {
    multi_evm_input.assert_coherency();

    let envs = create_envs_from_input(multi_evm_input);
    let cached_envs = CachedEvmEnv::from_envs(envs);

    let evm_call_result = TravelCallExecutor::new(&cached_envs)
        .call(&call, start_execution_location)
        .expect("travel call execution failed")
        .output;

    let start_env = cached_envs
        .get(start_execution_location)
        .expect("cannot get start evm env");

    let call_assumptions = CallAssumptions::new(start_env.header(), call.to, call.selector());

    GuestOutput::new(call_assumptions, evm_call_result)
}
