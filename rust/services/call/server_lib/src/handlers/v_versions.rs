use jsonrpsee::{IntoResponse, ResponsePayload};
use risc0_zkvm::get_version;
use serde::{Deserialize, Serialize};

use super::Config;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Versions {
    call_guest_id: String,
    chain_guest_id: String,
    api_version: String,
    risc0_version: String,
}

impl IntoResponse for Versions {
    type Output = Versions;

    fn into_response(self) -> ResponsePayload<'static, Self::Output> {
        ResponsePayload::success(self)
    }
}

#[allow(clippy::expect_used)]
pub fn v_versions(config: &Config) -> Versions {
    Versions {
        call_guest_id: config.call_guest_id_hex(),
        chain_guest_id: config.chain_guest_id_hex(),
        api_version: config.semver.clone(),
        risc0_version: get_version()
            .expect("Failed to parse risc0_zkvm version — CARGO_PKG_VERSION is not valid semver")
            .to_string(),
    }
}
