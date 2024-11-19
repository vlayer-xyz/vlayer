use std::sync::Arc;

use call_engine::{evm::env::location::ExecutionLocation, Call as EngineCall};
use call_host::{Error as HostError, Host};
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
    let hash = generate_hash(host.start_execution_location(), &call)?;
    info!("Calculated hash: {}", hash);
    let host_output = host.main(call).await?;
    Ok(CallResult::new(hash, host_output)?)
}

fn generate_hash(
    _execution_location: ExecutionLocation,
    _call: &EngineCall,
) -> Result<String, HostError> {
    use std::fmt::Write;

    use rand::{distributions::Alphanumeric, thread_rng, Rng};

    let input: Vec<u8> = thread_rng().sample_iter(&Alphanumeric).take(40).collect();
    let mut hasher = Sha256::new();
    hasher.update(&input);
    let digest = hasher.finalize();

    let mut encoded = String::with_capacity(digest.len() * 2 + 2);
    write!(&mut encoded, "0x{:x}", digest).map_err(|_| {
        HostError::ReturnHashEncoding(format!("failed to encode return hash as hex: {:x?}", digest))
    })?;
    Ok(encoded)
}
