use axum_jrpc::{JrpcResult, JsonRpcExtractor, JsonRpcResponse};

use crate::handlers::JsonRpcHandler;

pub(crate) async fn handle<H>(request: JsonRpcExtractor, handler: H) -> JrpcResult
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

pub(crate) async fn route(request: JsonRpcExtractor, handler: impl JsonRpcHandler) -> JrpcResult {
    let method = request.method();
    match method {
        "v_call" => handle(request, handler).await,
        _ => Err(request.method_not_found(method)),
    }
}
