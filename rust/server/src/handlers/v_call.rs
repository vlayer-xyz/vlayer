use crate::error::AppError;
use host::host::{Host, HostConfig};
use host::{Call as HostCall, ExecutionLocation};
use types::{Call, CallContext, CallResult};

pub mod types;

const LOCALHOST_RPC_URL: &str = "http://localhost:8545";

pub(crate) async fn call(params: (Call, CallContext)) -> Result<CallResult, AppError> {
    let call: HostCall = params.0.try_into()?;
    let context = params.1;

    let execution_location = ExecutionLocation::new(context.block_no, context.chain_id);
    let config = HostConfig::new(LOCALHOST_RPC_URL, execution_location);

    let return_data = tokio::task::spawn_blocking(|| Host::try_new(config)?.run(call)).await??;

    Ok(CallResult {
        result: format!(
            "start_contract_address: {}, function_selector: {}, evm_call_result: {:?}",
            return_data
                .guest_output
                .execution_commitment
                .startContractAddress,
            return_data
                .guest_output
                .execution_commitment
                .functionSelector,
            return_data.guest_output.evm_call_result
        ),
    })
}
