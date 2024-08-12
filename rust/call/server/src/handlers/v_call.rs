use std::sync::Arc;

use crate::error::AppError;
use crate::server::ServerConfig;
use async_trait::async_trait;
use axum_jrpc::{JrpcResult, JsonRpcExtractor, JsonRpcResponse};
use call_engine::io::Augmentors;
use call_host::host::{config::HostConfig, Host};
use call_host::Call as HostCall;
use serde::{Deserialize, Serialize};
use serde_json::json;
use types::{Call, CallContext, CallResult};

use super::JsonRpcHandler;

pub mod types;

pub struct VCall {
    config: Arc<ServerConfig>,
}

impl VCall {
    pub fn new(config: Arc<ServerConfig>) -> Self {
        Self { config }
    }

    pub async fn call_inner(&self, params: Params) -> Result<CallResult, AppError> {
        let call: HostCall = params.call.try_into()?;

        let host_config = HostConfig {
            rpc_urls: self.config.rpc_urls.clone(),
            start_chain_id: params.context.chain_id,
        };
    
        let return_data =
            tokio::task::spawn_blocking(|| Host::try_new(host_config)?.run(call, params.augmentors))
                .await??;
    
        Ok(CallResult {
            result: json!({
                "evm_call_result": return_data.guest_output.evm_call_result,
                "function_selector": return_data.guest_output.execution_commitment.functionSelector,
                "prover_contract_address": return_data.guest_output.execution_commitment.proverContractAddress,
                "seal": return_data.seal
            }),
        })
    }
}

#[async_trait]
impl JsonRpcHandler for VCall {
    async fn call(&self, request: JsonRpcExtractor) -> JrpcResult {
        let request_id = request.get_answer_id();
        let params: Params = request.parse_params()?;

        Ok(match self.call_inner(params).await {
            Ok(result) => JsonRpcResponse::success(request_id, result),
            Err(err) => JsonRpcResponse::error(request_id, err.into()),
        })
    }
}

#[derive(Deserialize, Serialize)]
pub struct Params {
    call: Call,
    context: CallContext,
    #[serde(default)]
    augmentors: Option<Augmentors>,
}
