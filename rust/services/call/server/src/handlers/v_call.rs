use std::sync::Arc;

use call_host::Host;
use serde::{Deserialize, Serialize};
use types::{Call, CallContext, CallResult};

use crate::{config::Config as ServerConfig, error::AppError};

pub mod types;

#[derive(Deserialize, Serialize, Debug)]
pub struct Params {
    call: Call,
    context: CallContext,
}

pub async fn v_call(config: Arc<ServerConfig>, params: Params) -> Result<CallResult, AppError> {
    let call = params.call.try_into()?;
    let host_config = config.get_host_config(params.context.chain_id);
    let call_result = Host::try_new(host_config)?.main(call).await?.try_into()?;

    Ok(call_result)
}
