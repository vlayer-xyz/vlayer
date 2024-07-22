use std::sync::Arc;

use crate::error::AppError;
use crate::server::Config;
use host::host::{config::HostConfig, Host};
use host::{Call as HostCall, ExecutionLocation};
use types::{Augmentors, Call, CallContext, CallResult};
use serde::{Deserialize, Serialize};

pub mod types;

#[derive(Deserialize, Serialize)]
pub struct Params(
    Call, 
    CallContext,
    #[serde(default)] Option<Augmentors>
);

pub(crate) async fn call(
    params: Params,
    config: Arc<Config>,
) -> Result<CallResult, AppError> {
    let call: HostCall = params.0.try_into()?;
    let context = params.1;

    let execution_location = ExecutionLocation::new(context.block_no, context.chain_id);
    let config = HostConfig::new(&config.url, execution_location);

    let return_data = tokio::task::spawn_blocking(|| Host::try_new(config)?.run(call)).await??;

    Ok(CallResult {
        result: format!(
            "start_contract_address: {}, function_selector: {}, evm_call_result: {:?}, seal: {:?}",
            return_data
                .guest_output
                .execution_commitment
                .startContractAddress,
            return_data
                .guest_output
                .execution_commitment
                .functionSelector,
            return_data.guest_output.evm_call_result,
            return_data.seal
        ),
    })
}
