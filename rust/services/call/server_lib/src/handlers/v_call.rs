use alloy_primitives::ChainId;
use call_host::{Error as HostError, Host};
use provider::Address;
use tracing::{info, info_span, Instrument};
use types::{Call, CallContext, CallHash, Result as VCallResult};

use super::{Params, SharedConfig, SharedProofs};
use crate::{
    gas_meter,
    proof::{self, Status as ProofStatus},
    Config,
};

pub mod types;

pub async fn v_call(
    config: SharedConfig,
    state: SharedProofs,
    call: Call,
    context: CallContext,
    params: Params,
) -> VCallResult<CallHash> {
    let call = call.parse_and_validate(config.max_calldata_size())?;

    let host = build_host(&config, context.chain_id, call.to).await?;
    let call_hash = (&host.start_execution_location(), &call).into();

    info!(hash = tracing::field::display(call_hash), "Call");

    let gas_meter_client =
        gas_meter::init(config.gas_meter_config(), call_hash, params.user_token, call.gas_limit)
            .await?;

    let mut found_existing = true;
    state.entry(call_hash).or_insert_with(|| {
        found_existing = false;
        ProofStatus::default()
    });

    if !found_existing {
        tokio::spawn(async move {
            let span = info_span!("http", id = params.req_id.to_string());
            proof::generate(
                call,
                host,
                gas_meter_client,
                state.clone(),
                call_hash,
                config.chain_proof_config(),
            )
            .instrument(span)
            .await
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
        .build(config.into())?;
    Ok(host)
}
