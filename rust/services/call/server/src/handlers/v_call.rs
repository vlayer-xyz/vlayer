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

    let host_config = config.into_host_config(params.context.chain_id);

    Host::try_new(&host_config)?
        .main(call)
        .await?
        .try_into()
        .map_err(AppError::Host)
}
