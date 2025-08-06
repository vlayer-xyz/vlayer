use std::sync::Arc;

use alloy_primitives::ChainId;
use call_host::{BuilderError, Call as EvmCall, Host};
use provider::Address;
use tracing::{Instrument, info, info_span};
use types::{Call, CallContext, CallHash, Result as VCallResult};

use super::{Params, State};
use crate::{
    Config, gas_meter,
    proof::{self, Status as ProofStatus},
};

pub mod types;

// Limit for the gas used during the preflight. It's not used for limiting cycles.
pub const EVM_GAS_LIMIT: u64 = 100_000_000;

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

    let vgas_limit = call.vgas_limit;
    let evm_call: EvmCall = call.parse_and_validate(config.max_calldata_size, EVM_GAS_LIMIT)?;

    let host = build_host(&config, context.chain_id, evm_call.to).await?;
    let call_hash = (&host.start_execution_location(), &evm_call).into();

    info!(hash = tracing::field::display(call_hash), "Call");

    let gas_meter_client = gas_meter::init(config.gas_meter_config.clone(), call_hash, token);

    let mut found_existing = true;
    state.entry(call_hash).or_insert_with(|| {
        found_existing = false;
        ProofStatus::default()
    });

    if !found_existing {
        let preflight_timeout = config.preflight_timeout;
        tokio::spawn(async move {
            let span = info_span!("http", id = req_id.to_string());
            proof::generator::Generator::new(
                gas_meter_client,
                vgas_limit,
                Arc::clone(&state),
                call_hash,
                preflight_timeout,
            )
            .run(host, evm_call)
            .instrument(span)
            .await;
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
        .with_rpc_urls(&config.rpc_urls)
        .with_chain_guest_id(config.chain_guest_id())
        .with_chain_client_config(config.chain_client_config.clone())?
        .with_start_chain_id(chain_id)?
        .with_prover_contract_addr(prover_contract_addr)
        .await?
        .build(config.into())?;
    Ok(host)
}
