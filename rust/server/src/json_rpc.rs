use std::sync::Arc;

use axum_jrpc::{JrpcResult, JsonRpcExtractor, JsonRpcResponse};
use tracing::instrument;

use crate::{handlers::v_call::call, server::Config};

#[instrument(level = "debug")]
pub(crate) async fn json_rpc(config: Arc<Config>, request: JsonRpcExtractor) -> JrpcResult {
    let request_id = request.get_answer_id();
    let method = request.method();
    let response = match method {
        "v_call" => {
            let params = request.parse_params()?;
            match call(params, config).await {
                Ok(result) => JsonRpcResponse::success(request_id, result),
                Err(err) => JsonRpcResponse::error(request_id, err.into()),
            }
        }
        _ => request.method_not_found(method),
    };
    Ok(response)
}
