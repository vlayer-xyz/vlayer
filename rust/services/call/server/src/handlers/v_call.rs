use std::sync::Arc;

use alloy_primitives::hex::ToHexExt;
use call_engine::{evm::env::location::ExecutionLocation, Call as EngineCall};
use call_host::Host;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tracing::info;
use types::{Call, CallContext, CallResult};

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
    let hash = generate_hash(host.start_execution_location(), &call);
    info!("Calculated hash: {}", hash.encode_hex_with_prefix());
    let host_output = host.main(call).await?;
    Ok(CallResult::new(hash, host_output)?)
}

fn generate_hash(execution_location: ExecutionLocation, call: &EngineCall) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(execution_location.block_number.to_le_bytes());
    hasher.update(execution_location.chain_id.to_le_bytes());
    hasher.update(call.to);
    hasher.update(&call.data);
    hasher.finalize().as_slice().into()
}
