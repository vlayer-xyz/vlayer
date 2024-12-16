use call_engine::{Call as EngineCall, HostOutput};
use call_host::Host;
use common::Hashable;
use tracing::info;
use types::{Call, CallContext, CallHash, CallHashData};

use super::{QueryParams, SharedConfig, SharedProofs};
use crate::{
    error::AppError,
    gas_meter::{self, Client as GasMeterClient, ComputationStage},
    handlers::ProofStatus,
};

pub mod types;

pub async fn v_call(
    config: SharedConfig,
    state: SharedProofs,
    params: QueryParams,
    call: Call,
    context: CallContext,
) -> Result<CallHash, AppError> {
    info!("v_call => {call:#?} {context:#?}");
    let call: EngineCall = call.try_into()?;

    let host = Host::builder()
        .with_rpc_urls(config.rpc_urls())
        .with_start_chain_id(context.chain_id)?
        .with_chain_proof_url(config.chain_proof_url())
        .await?
        .with_prover_contract_addr(call.to)
        .await?
        .build(config.as_ref().into());
    let call_hash = CallHashData::new(host.start_execution_location(), call.clone())
        .hash_slow()
        .into();
    info!("Calculated hash: {}", call_hash);

    let gas_meter_client: Box<dyn GasMeterClient> = config
        .gas_meter_config()
        .map_or(Box::new(gas_meter::NoOpClient), |config| {
            Box::new(gas_meter::RpcClient::new(config, call_hash, params.token))
        });

    gas_meter_client.allocate(context.gas_limit).await?;

    tokio::spawn(async move {
        let res = generate_proof(call, host, gas_meter_client).await;
        state.insert(call_hash, ProofStatus::Ready(res));
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
