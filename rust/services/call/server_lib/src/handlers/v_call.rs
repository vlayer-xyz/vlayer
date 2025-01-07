use std::time::Duration;

use alloy_primitives::ChainId;
use call_engine::Call as EngineCall;
use call_host::{Error as HostError, Host};
use provider::Address;
use tracing::info;
use types::{Call, CallContext, CallHash};

use super::{QueryParams, SharedConfig, SharedProofs, UserToken};
use crate::{
    error::AppError,
    gas_meter::{self, Client as GasMeterClient, ComputationStage},
    handlers::{Metrics, ProofReceipt, ProofStatus, RawData, Times},
    ChainProofConfig, Config,
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
    let call = call.parse_and_validate(config.max_calldata_size())?;

    let host = build_host(&config, context.chain_id, call.to).await?;
    let call_hash = (&host.start_execution_location(), &call).into();
    info!(
        "Start execution location: {:?} call hash: {call_hash}",
        host.start_execution_location()
    );
    let gas_meter_client = init_gas_meter(&config, call_hash, params.token, call.gas_limit).await?;

    let mut found_existing = true;
    state.entry(call_hash).or_insert_with(|| {
        found_existing = false;
        ProofStatus::Queued
    });

    if !found_existing {
        tokio::spawn(async move {
            let res = generate_proof(
                call,
                host,
                gas_meter_client,
                state.clone(),
                call_hash,
                config.chain_proof_config(),
            )
            .await;
            state.insert(call_hash, ProofStatus::Ready(res));
        });
    }

    Ok(call_hash)
}

async fn build_host(
    config: &Config,
    chain_id: ChainId,
    prover_contract_addr: Address,
) -> Result<Host, HostError> {
    let host = Host::builder()
        .with_rpc_urls(config.rpc_urls())
        .with_chain_guest_id(config.chain_guest_id())
        .with_chain_proof_url(config.chain_proof_url())?
        .with_start_chain_id(chain_id)?
        .with_prover_contract_addr(prover_contract_addr)
        .await
        .map_err(HostError::Builder)?
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
    chain_proof_config: Option<ChainProofConfig>,
) -> Result<ProofReceipt, AppError> {
    info!("Processing call {call_hash}");

    await_chain_proof_ready(&host, &state, call_hash, chain_proof_config).await?;

    info!("Executing preflight for call {call_hash}");

    state.insert(call_hash, ProofStatus::Preflight);
    let prover = host.prover();
    let call_guest_id = host.call_guest_id();
    let preflight_result = host.preflight(call).await.map_err(HostError::Preflight)?;
    let gas_used = preflight_result.gas_used;
    let preflight_time = preflight_result.elapsed_time.as_millis().try_into()?;

    gas_meter_client
        .refund(ComputationStage::Preflight, gas_used)
        .await?;

    info!("Generating proof for call {call_hash}");
    state.insert(call_hash, ProofStatus::Proving);
    let host_output =
        Host::prove(&prover, call_guest_id, preflight_result).map_err(HostError::Proving)?;
    let cycles_used = host_output.cycles_used;
    let proving_time = host_output.elapsed_time.as_millis().try_into()?;
    let raw_data: RawData = host_output.try_into()?;

    gas_meter_client
        .refund(ComputationStage::Proving, gas_used)
        .await?;

    let times = Times::new(preflight_time, proving_time);
    let metrics = Metrics::new(gas_used, cycles_used, times);
    Ok(ProofReceipt::new(raw_data, metrics))
}

async fn await_chain_proof_ready(
    host: &Host,
    state: &SharedProofs,
    call_hash: CallHash,
    config: Option<ChainProofConfig>,
) -> Result<(), AppError> {
    if let Some(ChainProofConfig {
        poll_interval,
        timeout,
        ..
    }) = config
    {
        let poll_interval = Duration::from_secs(poll_interval);
        let timeout = Duration::from_secs(timeout);
        // Wait for chain proof if necessary
        let start = tokio::time::Instant::now();
        while !host
            .chain_proof_ready()
            .await
            .map_err(HostError::AwaitingChainProof)?
        {
            info!(
                "Location {:?} not indexed. Waiting for chain proof",
                host.start_execution_location()
            );
            state.insert(call_hash, ProofStatus::WaitingForChainProof);
            tokio::time::sleep(poll_interval).await;
            if start.elapsed() > timeout {
                return Err(AppError::ChainProofTimeout);
            }
        }
    }
    Ok(())
}
