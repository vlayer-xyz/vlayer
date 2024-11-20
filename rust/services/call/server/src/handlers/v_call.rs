use std::sync::Arc;

use call_engine::Call as EngineCall;
use call_host::Host;
use common::Hashable;
use serde::{Deserialize, Serialize};
use tracing::info;
use types::{Call, CallContext, CallHashData, CallResult};

use crate::{config::Config as ServerConfig, error::AppError};

pub mod types;

#[derive(Deserialize, Serialize, Debug)]
pub struct Params {
    call: Call,
    context: CallContext,
}

pub async fn v_call(config: Arc<ServerConfig>, params: Params) -> Result<CallResult, AppError> {
    let call: EngineCall = params.call.try_into()?;
    let host_config = config.get_host_config(params.context.chain_id);
    let host = Host::try_new(host_config)?;
    let call_hash = CallHashData::new(host.start_execution_location(), call.clone())
        .hash_slow()
        .into();
    info!("Calculated hash: {}", call_hash);
    let host_output = host.main(call).await?;
    Ok(CallResult::try_new(call_hash, host_output)?)
}
