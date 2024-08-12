use crate::JsonRpcHandler;
use axum_jrpc::{JrpcResult, JsonRpcExtractor, JsonRpcResponse};

async fn handle<H>(request: JsonRpcExtractor, handler: H) -> JrpcResult
where
    H: JsonRpcHandler,
{
    let request_id = request.get_answer_id();
    let params: H::Params = request.parse_params()?;

    Ok(match handler.call(params).await {
        Ok(result) => JsonRpcResponse::success(request_id, result),
        Err(err) => JsonRpcResponse::error(request_id, err.into()),
    })
}

pub async fn route(
    request: JsonRpcExtractor,
    method: &str,
    handler: impl JsonRpcHandler,
) -> JrpcResult {
    let called_method = request.method();
    if called_method == method {
        handle(request, handler).await
    } else {
        Err(request.method_not_found(called_method))
    }
}
