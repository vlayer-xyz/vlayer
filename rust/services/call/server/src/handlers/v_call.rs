use std::sync::Arc;

use call_host::{
    host::{config::HostConfig, Host},
    Call as HostCall,
};
use serde::{Deserialize, Serialize};
use types::{Call, CallContext, CallResult};

use crate::{config::ServerConfig, error::AppError};

pub mod types;

#[derive(Deserialize, Serialize, Debug)]
pub struct Params {
    call: Call,
    context: CallContext,
}

pub async fn v_call(config: Arc<ServerConfig>, params: Params) -> Result<CallResult, AppError> {
    let call: HostCall = params.call.try_into()?;

    let host_config = HostConfig {
        rpc_urls: config.rpc_urls.clone(),
        start_chain_id: params.context.chain_id,
        proof_mode: config.proof_mode.into(),
        chain_proof_url: config.chain_proof_url.clone(),
        max_calldata_size: config.max_request_size,
    };

    let return_data = Host::try_new(&host_config)?.run(call).await?;

    return_data.try_into().map_err(AppError::Host)
}
