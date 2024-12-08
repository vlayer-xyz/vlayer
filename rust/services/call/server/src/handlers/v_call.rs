use std::sync::Arc;

use call_engine::Call as EngineCall;
use call_host::Host;
use common::Hashable;
use serde::{Deserialize, Serialize};
use tracing::info;
use types::{Call, CallContext, CallHash, CallHashData};

use super::{generate_proof, SharedState};
use crate::{
    config::Config as ServerConfig,
    error::AppError,
    gas_meter::{Client, Config as GasMeterConfig},
};

pub mod types;

#[derive(Deserialize, Serialize, Debug)]
pub struct Params {
    call: Call,
    context: CallContext,
}

pub async fn v_call(
    config: Arc<ServerConfig>,
    state: SharedState,
    params: Params,
) -> Result<CallHash, AppError> {
    info!("v_call => {params:#?}");
    let call: EngineCall = params.call.try_into()?;
    let host_config = config.get_host_config(params.context.chain_id);
    let host = Host::try_new(host_config).await?;
    let call_hash = CallHashData::new(host.start_execution_location(), call.clone())
        .hash_slow()
        .into();
    info!("Calculated hash: {}", call_hash);

    let gas_meter_client = config
        .gas_meter_config()
        .map(|GasMeterConfig { url, time_to_live }| Client::new(&url, call_hash, time_to_live));

    if let Some(client) = gas_meter_client.as_ref() {
        client.allocate_gas(params.context.gas_limit).await?;
    }

    let handle = tokio::spawn(async move {
        let res = generate_proof(call, host, gas_meter_client).await;
        state.write().insert(call_hash, res);
    });

    #[cfg(feature = "testing")]
    handle.await.unwrap();
    #[cfg(not(feature = "testing"))]
    drop(handle);

    Ok(call_hash)
}
