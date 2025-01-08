use alloy_primitives::ChainId;
use call_engine::Call as EngineCall;
use call_host::{Error as HostError, Host};
use provider::Address;
use tracing::info;
use types::{Call, CallContext, CallHash, Result as VCallResult};

use super::{QueryParams, SharedConfig, SharedProofs};
use crate::{
    chain_proof::{self, Config as ChainProofConfig},
    gas_meter::{self, Client as GasMeterClient},
    handlers::{Metrics, ProofReceipt, ProofStatus},
    preflight, proving,
    v_get_proof_receipt::Result as VGetProofReceiptResult,
    Config,
};

pub mod types;

pub async fn v_call(
    config: SharedConfig,
    state: SharedProofs,
    params: QueryParams,
    call: Call,
    context: CallContext,
) -> VCallResult<CallHash> {
    info!("v_call => {call:#?} {context:#?}");
    let call = call.parse_and_validate(config.max_calldata_size())?;

    let host = build_host(&config, context.chain_id, call.to).await?;
    let call_hash = (&host.start_execution_location(), &call).into();
    info!(
        "Start execution location: {:?} call hash: {call_hash}",
        host.start_execution_location()
    );
    let gas_meter_client =
        gas_meter::init(config.gas_meter_config(), call_hash, params.token, call.gas_limit).await?;

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
) -> std::result::Result<Host, HostError> {
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

async fn generate_proof(
    call: EngineCall,
    host: Host,
    gas_meter_client: impl GasMeterClient,
    state: SharedProofs,
    call_hash: CallHash,
    chain_proof_config: Option<ChainProofConfig>,
) -> VGetProofReceiptResult<ProofReceipt> {
    let mut metrics = Metrics::default();

    let prover = host.prover();
    let call_guest_id = host.call_guest_id();

    info!("Processing call {call_hash}");

    chain_proof::await_ready(&host, &state, call_hash, chain_proof_config).await?;

    let preflight_result =
        preflight::await_preflight(host, &state, call, call_hash, &gas_meter_client, &mut metrics)
            .await?;

    let raw_data = proving::await_proving(
        &prover,
        &state,
        call_hash,
        call_guest_id,
        preflight_result,
        &gas_meter_client,
        &mut metrics,
    )
    .await?;

    Ok(ProofReceipt::new(raw_data, metrics))
}
