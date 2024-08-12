use std::sync::Arc;

use crate::error::AppError;
use crate::server::ServerConfig;
use call_engine::io::Augmentors;
use call_host::host::{config::HostConfig, Host};
use call_host::Call as HostCall;
use serde::{Deserialize, Serialize};
use serde_json::json;
use server_utils::JsonRpcHandler;
use types::{Call, CallContext, CallResult};

pub mod types;

pub struct VCall {
    config: Arc<ServerConfig>,
}

impl VCall {
    pub fn new(config: Arc<ServerConfig>) -> Self {
        Self { config }
    }
}

#[allow(refining_impl_trait)]
impl JsonRpcHandler for VCall {
    type Params = Params;
    type Result = CallResult;

    async fn call(&self, params: Self::Params) -> Result<Self::Result, AppError> {
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

#[derive(Deserialize, Serialize)]
pub struct Params {
    call: Call,
    context: CallContext,
    #[serde(default)]
    augmentors: Option<Augmentors>,
}
