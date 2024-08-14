use std::{future::Future, pin::Pin};

use axum_jrpc::{error::JsonRpcError, JrpcResult, JsonRpcExtractor, JsonRpcResponse};
use serde::{de::DeserializeOwned, Serialize};

async fn handle<Params, Return, Error>(
    request: JsonRpcExtractor,
    handler: impl Fn(Params) -> Pin<Box<dyn Future<Output = Result<Return, Error>> + Send + 'static>>,
) -> JrpcResult
where
    Params: DeserializeOwned,
    Return: Serialize,
    Error: Into<JsonRpcError>,
{
    let request_id = request.get_answer_id();
    let params = request.parse_params()?;

    Ok(match handler(params).await {
        Ok(result) => JsonRpcResponse::success(request_id, result),
        Err(err) => JsonRpcResponse::error(request_id, err.into()),
    })
}

pub async fn route<Params, Return, Error>(
    request: JsonRpcExtractor,
    method: &str,
    handler: impl Fn(Params) -> Pin<Box<dyn Future<Output = Result<Return, Error>> + Send + 'static>>,
) -> JrpcResult
where
    Params: DeserializeOwned,
    Return: Serialize,
    Error: Into<JsonRpcError>,
{
    let called_method = request.method();
    if called_method == method {
        handle(request, handler).await
    } else {
        Err(request.method_not_found(called_method))
    }
}
