use std::sync::Arc;

use call_engine::{Call as EngineCall, HostOutput};
use call_host::Host;
use common::Hashable;
use serde::{Deserialize, Serialize};
use tracing::info;
use types::{Call, CallContext, CallHash, CallHashData};

use super::SharedState;
use crate::{
    config::Config as ServerConfig,
    error::AppError,
    gas_meter::{self, Client as GasMeterClient, ComputationStage},
};

pub mod types;

#[derive(Deserialize, Serialize, Debug)]
pub struct Params {
    pub call: Call,
    pub context: CallContext,
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

    let gas_meter_client: Box<dyn GasMeterClient> = config
        .gas_meter_config()
        .map_or(Box::new(gas_meter::NoOpClient), |config| {
            Box::new(gas_meter::RpcClient::new(config, call_hash))
        });

    gas_meter_client.allocate(params.context.gas_limit).await?;

    tokio::spawn(async move {
        let res = generate_proof(call, host, gas_meter_client).await;
        state.insert(call_hash, res);
    });

    Ok(call_hash)
}

async fn generate_proof(
    call: EngineCall,
    host: Host,
    gas_meter_client: impl GasMeterClient,
) -> Result<HostOutput, AppError> {
    let prover = host.prover();
    let call_guest_id = host.call_guest_id();
    let preflight_result = host.preflight(call).await?;
    let gas_used = preflight_result.gas_used;

    gas_meter_client
        .refund(ComputationStage::Preflight, gas_used)
        .await?;

    let host_output = Host::prove(&prover, call_guest_id, preflight_result)?;

    gas_meter_client
        .refund(ComputationStage::Proving, gas_used)
        .await?;

    Ok(host_output)
}
