use std::sync::Arc;

use crate::error::AppError;
use crate::server::ServerConfig;
use host::host::{config::HostConfig, Host};
use host::Call as HostCall;
use serde::{Deserialize, Serialize};
use types::{Augmentors, Call, CallContext, CallResult};

pub mod types;

#[derive(Deserialize, Serialize)]
pub struct Params {
    call: Call,
    context: CallContext,
    #[serde(default)]
    augmentors: Option<Augmentors>,
}

pub(crate) async fn call(
    params: Params,
    config: Arc<ServerConfig>,
) -> Result<CallResult, AppError> {
    let call: HostCall = params.call.try_into()?;

    let host_config = HostConfig {
        rpc_urls: config.rpc_urls.clone(),
        chain_id: params.context.chain_id,
    };

    let return_data =
        tokio::task::spawn_blocking(|| Host::try_new(host_config)?.run(call)).await??;

    Ok(CallResult {
        result: format!(
            "prover_contract_address: {}, function_selector: {}, evm_call_result: {:?}, seal: {:?}",
            return_data
                .guest_output
                .execution_commitment
                .proverContractAddress,
            return_data
                .guest_output
                .execution_commitment
                .functionSelector,
            return_data.guest_output.evm_call_result,
            return_data.seal
        ),
    })
}
