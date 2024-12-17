use std::time::Duration;

use alloy_primitives::ChainId;
use call_engine::{Call as EngineCall, HostOutput};
use call_host::Host;
use provider::Address;
use tracing::info;
use types::{Call, CallContext, CallHash};

use super::{QueryParams, SharedConfig, SharedProofs, UserToken};
use crate::{
    error::AppError,
    gas_meter::{self, Client as GasMeterClient, ComputationStage},
    handlers::ProofStatus,
    Config,
};

pub mod types;

const CHAIN_PROOF_POLL_INTERVAL: Duration = Duration::from_secs(1);
const CHAIN_PROOF_TIMEOUT: Duration = Duration::from_secs(60);

pub async fn v_call(
    config: SharedConfig,
    state: SharedProofs,
    params: QueryParams,
    call: Call,
    context: CallContext,
) -> Result<CallHash, AppError> {
    info!("v_call => {call:#?} {context:#?}");
    let call: EngineCall = call.try_into()?;

    let host = build_host(&config, context.chain_id, call.to).await?;
    let call_hash = (&host.start_execution_location(), &call).into();
    info!("Calculated hash: {}", call_hash);
    let gas_meter_client =
        init_gas_meter(&config, call_hash, params.token, context.gas_limit).await?;
    state.insert(call_hash, ProofStatus::Pending);

    tokio::spawn(async move {
        let res = generate_proof(call, host, gas_meter_client, state.clone(), call_hash).await;
        state.insert(call_hash, ProofStatus::Ready(res));
    });

    Ok(call_hash)
}

async fn build_host(
    config: &Config,
    chain_id: ChainId,
    prover_contract_addr: Address,
) -> Result<Host, AppError> {
    let host = Host::builder()
        .with_rpc_urls(config.rpc_urls())
        .with_start_chain_id(chain_id)?
        .with_chain_proof_url(config.chain_proof_url())
        .await?
        .with_prover_contract_addr(prover_contract_addr)
        .await?
        .build(config.into());
    Ok(host)
}

async fn init_gas_meter(
    config: &Config,
    call_hash: CallHash,
    user_token: Option<UserToken>,
    gas_limit: u64,
) -> Result<Box<dyn GasMeterClient>, AppError> {
    let gas_meter_client: Box<dyn GasMeterClient> = config
        .gas_meter_config()
        .map_or(Box::new(gas_meter::NoOpClient), |config| {
            Box::new(gas_meter::RpcClient::new(config, call_hash, user_token))
        });

    gas_meter_client.allocate(gas_limit).await?;
    Ok(gas_meter_client)
}

async fn generate_proof(
    call: EngineCall,
    host: Host,
    gas_meter_client: impl GasMeterClient,
    state: SharedProofs,
    call_hash: CallHash,
) -> Result<HostOutput, AppError> {
    // Wait for chain proof if necessary
    let start = tokio::time::Instant::now();
    while !host.chain_proof_ready().await? {
        state.insert(call_hash, ProofStatus::WaitingForChainProof);
        tokio::time::sleep(CHAIN_PROOF_POLL_INTERVAL).await;
        if start.elapsed() > CHAIN_PROOF_TIMEOUT {
            return Err(AppError::ChainProofTimeout);
        }
    }

    state.insert(call_hash, ProofStatus::Preflight);
    let prover = host.prover();
    let call_guest_id = host.call_guest_id();
    let preflight_result = host.preflight(call).await?;
    let gas_used = preflight_result.gas_used;

    gas_meter_client
        .refund(ComputationStage::Preflight, gas_used)
        .await?;

    state.insert(call_hash, ProofStatus::Proving);
    let host_output = Host::prove(&prover, call_guest_id, preflight_result)?;

    gas_meter_client
        .refund(ComputationStage::Proving, gas_used)
        .await?;

    Ok(host_output)
}
