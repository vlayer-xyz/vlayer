use alloy_primitives::ChainId;
use call_host::{BuilderError, Host};
use provider::Address;
use tracing::{Instrument, info, info_span};
use types::{Call, CallContext, CallHash, Result as VCallResult};

use super::{Params, State};
use crate::{
    Config, gas_meter,
    proof::{self, Status as ProofStatus},
};

pub mod types;

pub async fn v_call(
    state: State,
    call: Call,
    context: CallContext,
    params: Params,
) -> VCallResult<CallHash> {
    let Params {
        config,
        token,
        req_id,
    } = params;

    let call = call.parse_and_validate(config.max_calldata_size)?;

    let host = build_host(&config, context.chain_id, call.to).await?;
    let call_hash = (&host.start_execution_location(), &call).into();

    info!(hash = tracing::field::display(call_hash), "Call");

    let gas_meter_client = gas_meter::init(config.gas_meter_config.clone(), call_hash, token);

    let mut found_existing = true;
    state.entry(call_hash).or_insert_with(|| {
        found_existing = false;
        ProofStatus::default()
    });

    if !found_existing {
        tokio::spawn(async move {
            let span = info_span!("http", id = req_id.to_string());
            proof::generate(call, host, gas_meter_client, state.clone(), call_hash)
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
) -> std::result::Result<Host, BuilderError> {
    let host = Host::builder()
        .with_rpc_urls(config.rpc_urls.clone())
        .with_chain_guest_id(config.chain_guest_id())
        .with_chain_client_config(config.chain_client_config.clone())?
        .with_start_chain_id(chain_id)?
        .with_prover_contract_addr(prover_contract_addr)
        .await?
        .build(config.into())?;
    Ok(host)
}
