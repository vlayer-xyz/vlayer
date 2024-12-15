use serde::{Deserialize, Serialize};

use crate::{config::Config as ServerConfig, error::AppError};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Versions {
    call_guest_id: String,
    chain_guest_id: String,
    api_version: String,
}

pub fn v_versions(config: &ServerConfig) -> Result<Versions, AppError> {
    Ok(Versions {
        call_guest_id: config.call_guest_id(),
        chain_guest_id: config.chain_guest_id(),
        api_version: config.api_version(),
    })
}
