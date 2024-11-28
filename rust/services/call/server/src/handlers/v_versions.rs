use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::{config::Config as ServerConfig, error::AppError};

#[derive(Deserialize, Serialize, Debug)]
pub struct Versions {
    call_guest_id: String,
    chain_guest_id: String,
    api_version: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Params {}

pub async fn v_versions(config: Arc<ServerConfig>, _: Params) -> Result<Versions, AppError> {
    Ok(Versions {
        call_guest_id: config.call_guest_id(),
        chain_guest_id: config.chain_guest_id(),
        api_version: config.api_version(),
    })
}
