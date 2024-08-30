use std::sync::Arc;

use crate::config::ServerConfig;
use crate::error::AppError;
use call_host::host::{config::HostConfig, Host};
use call_host::Call as HostCall;
use serde::{Deserialize, Serialize};
use types::{Call, CallContext, CallResult};

pub mod types;

#[derive(Deserialize, Serialize)]
pub struct Params {
    call: Call,
    context: CallContext,
}

pub async fn v_call(config: Arc<ServerConfig>, params: Params) -> Result<CallResult, AppError> {
    let call: HostCall = params.call.try_into()?;

    let host_config = HostConfig {
        rpc_urls: config.rpc_urls.clone(),
        start_chain_id: params.context.chain_id,
        proof_mode: config.proof_mode.clone().into(),
    };

    let return_data =
        tokio::task::spawn_blocking(|| Host::try_new(host_config)?.run(call)).await??;

    Ok(return_data.into())
}

fn proof_to_json(proof: Proof) -> Value {
    json!({
        "length": u256_to_number(proof.length),
        "seal": {
            "verifierSelector": proof.seal.verifierSelector,
            "seal": proof.seal.seal,
            "mode": Into::<u8>::into(proof.seal.mode),
        },
        "commitment": {
            "functionSelector": proof.commitment.functionSelector,
            "proverContractAddress": proof.commitment.proverContractAddress,
            "settleBlockNumber": u256_to_number(proof.commitment.settleBlockNumber),
            "settleBlockHash": proof.commitment.settleBlockHash,
        }
    })
}

fn u256_to_number(value: U256) -> u64 {
    u64::try_from(value).expect("Expected value to fit into u64")
}
