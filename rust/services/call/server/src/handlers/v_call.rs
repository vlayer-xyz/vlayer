use std::sync::Arc;

use call_engine::Call as EngineCall;
use call_host::Host;
use common::Hashable;
use serde::{Deserialize, Serialize};
use tracing::info;
use types::{Call, CallContext, CallHash, CallHashData};

use crate::{
    config::Config as ServerConfig,
    error::AppError,
    gas_meter::{Client, ComputationStage, Config as GasMeterConfig},
};

pub mod types;

#[derive(Deserialize, Serialize, Debug)]
pub struct Params {
    call: Call,
    context: CallContext,
}

pub async fn v_call(config: Arc<ServerConfig>, params: Params) -> Result<CallHash, AppError> {
    info!("v_call => {params:#?}");
    let call: EngineCall = params.call.try_into()?;
    let host_config = config.get_host_config(params.context.chain_id);
    let host = Host::try_new(host_config)?;
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

    let prover = host.prover();
    let call_guest_id = host.call_guest_id();
    let preflight_result = host.preflight(call).await?;
    let gas_used = preflight_result.gas_used;

    if let Some(client) = gas_meter_client.as_ref() {
        client
            .refund_unused_gas(ComputationStage::Preflight, gas_used)
            .await?;
    }

    let host_output = Host::prove(&prover, call_guest_id, preflight_result)?;

    if let Some(client) = gas_meter_client {
        client
            .refund_unused_gas(ComputationStage::Proving, gas_used)
            .await?;
    }

    Ok(call_hash)
}
