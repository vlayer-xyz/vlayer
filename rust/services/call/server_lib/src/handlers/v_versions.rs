use jsonrpsee::{IntoResponse, ResponsePayload};
use serde::{Deserialize, Serialize};

use super::Config;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Versions {
    call_guest_id: String,
    chain_guest_id: String,
    api_version: String,
}

impl IntoResponse for Versions {
    type Output = Versions;

    fn into_response(self) -> ResponsePayload<'static, Self::Output> {
        ResponsePayload::success(self)
    }
}

pub fn v_versions(config: &Config) -> Versions {
    Versions {
        call_guest_id: config.call_guest_id_hex(),
        chain_guest_id: config.chain_guest_id_hex(),
        api_version: config.api_version(),
    }
}
