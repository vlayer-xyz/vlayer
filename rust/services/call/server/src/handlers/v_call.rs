use std::sync::Arc;

use call_host::host::Host;
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
    let call = params.call.try_into()?;
    let host_config = config.into_host_config(params.context.chain_id);
    let host = Host::try_new(host_config)?;
    let call_result = host.main(call).await?.try_into()?;

    Ok(call_result)
}
