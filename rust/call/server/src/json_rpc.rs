use std::sync::Arc;

use axum_jrpc::{JrpcResult, JsonRpcExtractor};
use tracing::instrument;

use crate::{
    handlers::{v_call::VCall, JsonRpcHandler},
    server::ServerConfig,
};

#[instrument(level = "debug")]
pub(crate) async fn json_rpc(config: Arc<ServerConfig>, request: JsonRpcExtractor) -> JrpcResult {
    let method = request.method();
    match method {
        "v_call" => VCall::new(config).call(request).await,
        _ => Err(request.method_not_found(method)),
    }
}
